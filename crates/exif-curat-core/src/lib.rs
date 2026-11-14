//! EXIF-CURAT core: find and strip metadata segments in JPEG files.
//!
//! Basis: ITU-T T.81 (JPEG) marker syntax, public. Parsers never panic on malformed input.

/// What can go wrong while walking a file.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotJpeg,
    Truncated,
    BadMarker(usize),
}

/// A marker segment before the image data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub marker: u8,
    /// Offset of the `0xFF` that starts the segment.
    pub offset: usize,
    /// Total length including the marker and the length field.
    pub len: usize,
}

/// What a metadata segment holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Exif,
    Xmp,
    Iptc,
    Comment,
    OtherApp1,
}

/// Walk the segments up to the start of the scan; returns them and the offset where the scan data begins.
pub fn segments(data: &[u8]) -> Result<(Vec<Segment>, usize), Error> {
    if data.len() < 2 || data[0] != 0xFF || data[1] != 0xD8 {
        return Err(Error::NotJpeg);
    }
    let mut segs = vec![Segment {
        marker: 0xD8,
        offset: 0,
        len: 2,
    }];
    let mut i = 2;
    loop {
        if i >= data.len() {
            return Err(Error::Truncated);
        }
        if data[i] != 0xFF {
            return Err(Error::BadMarker(i));
        }
        let start = i;
        while i < data.len() && data[i] == 0xFF {
            i += 1;
        }
        if i >= data.len() {
            return Err(Error::Truncated);
        }
        let m = data[i];
        i += 1;
        match m {
            0xDA => return Ok((segs, start)),
            0xD9 => {
                segs.push(Segment {
                    marker: m,
                    offset: start,
                    len: i - start,
                });
                return Ok((segs, data.len()));
            }
            0x00 => return Err(Error::BadMarker(start)),
            0x01 | 0xD0..=0xD7 => segs.push(Segment {
                marker: m,
                offset: start,
                len: i - start,
            }),
            _ => {
                if i + 2 > data.len() {
                    return Err(Error::Truncated);
                }
                let l = usize::from(u16::from_be_bytes([data[i], data[i + 1]]));
                if l < 2 || i + l > data.len() {
                    return Err(Error::Truncated);
                }
                segs.push(Segment {
                    marker: m,
                    offset: start,
                    len: i + l - start,
                });
                i += l;
            }
        }
    }
}

/// Classify a segment that carries metadata; `None` for structural segments we keep.
pub fn classify(data: &[u8], seg: &Segment) -> Option<Kind> {
    let payload = data
        .get(seg.offset + 4..seg.offset + seg.len)
        .unwrap_or(&[]);
    match seg.marker {
        0xFE => Some(Kind::Comment),
        0xED => Some(Kind::Iptc),
        0xE1 if payload.starts_with(b"Exif\0\0") => Some(Kind::Exif),
        0xE1 if payload.starts_with(b"http://ns.adobe.com/xap/1.0/\0") => Some(Kind::Xmp),
        0xE1 => Some(Kind::OtherApp1),
        _ => None,
    }
}

/// Every metadata segment found, in file order.
pub fn scan(data: &[u8]) -> Result<Vec<(Segment, Kind)>, Error> {
    let (segs, _) = segments(data)?;
    Ok(segs
        .into_iter()
        .filter_map(|s| classify(data, &s).map(|k| (s, k)))
        .collect())
}

/// A copy of the file without Exif/XMP (APP1), IPTC (APP13) and comment segments.
/// The ICC profile (APP2) and everything that affects decoding is kept.
pub fn strip(data: &[u8]) -> Result<Vec<u8>, Error> {
    let (segs, tail) = segments(data)?;
    let mut out = Vec::with_capacity(data.len());
    for s in &segs {
        if classify(data, s).is_none() {
            out.extend_from_slice(&data[s.offset..s.offset + s.len]);
        }
    }
    out.extend_from_slice(&data[tail..]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        let mut v = vec![0xFF, 0xD8];
        v.extend_from_slice(&[0xFF, 0xE0, 0, 16]);
        v.extend_from_slice(b"JFIF\0\x01\x01\0\0\x01\0\x01\0\0");
        v.extend_from_slice(&[0xFF, 0xE1, 0, 12]);
        v.extend_from_slice(b"Exif\0\0GPS!");
        v.extend_from_slice(&[0xFF, 0xFE, 0, 7]);
        v.extend_from_slice(b"hello");
        v.extend_from_slice(&[0xFF, 0xDA, 0, 2, 0xAB, 0xCD, 0xFF, 0xD9]);
        v
    }

    #[test]
    fn finds_metadata() {
        let kinds: Vec<Kind> = scan(&sample())
            .unwrap()
            .into_iter()
            .map(|(_, k)| k)
            .collect();
        assert_eq!(kinds, [Kind::Exif, Kind::Comment]);
    }

    #[test]
    fn strips_metadata_keeps_the_rest() {
        let out = strip(&sample()).unwrap();
        assert!(!out.windows(4).any(|w| w == b"Exif" || w == b"hell"));
        assert!(out.windows(4).any(|w| w == b"JFIF"));
        assert!(out.ends_with(&[0xFF, 0xDA, 0, 2, 0xAB, 0xCD, 0xFF, 0xD9]));
        assert_eq!(scan(&out).unwrap().len(), 0);
    }

    #[test]
    fn rejects_garbage_without_panicking() {
        assert_eq!(segments(b"PNG"), Err(Error::NotJpeg));
        assert_eq!(segments(&[0xFF, 0xD8]), Err(Error::Truncated));
        assert!(segments(&[0xFF, 0xD8, 0xFF, 0xE1, 0xFF, 0xFF]).is_err());
        assert!(segments(&[0xFF, 0xD8, 0x12]).is_err());
    }
}
