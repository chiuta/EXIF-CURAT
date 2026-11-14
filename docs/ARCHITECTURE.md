# EXIF-CURAT — architecture

One parser module per format in `exif-curat-core` (`jpeg`, later `png`, `pdf`, `ooxml`, `mp4`…), each with the same three functions:
`segments/scan` (find), `classify` (what is it), `strip` (remove). Parsers never panic and never allocate based on untrusted lengths without a check.
**Clean-room rule for this project:** formats are implemented from the public format specifications only. Tag tables and print conversions of other tools
(ExifTool and its ports) are *not* to be copied — see CLEANROOM.md. The aim is sanitising and auditing, not a complete metadata editor.
