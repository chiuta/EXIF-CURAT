# Related projects and where EXIF-CURAT fits

*Checked with a limited web search on 13 November 2026. Facts change quickly: re-check before announcing or depending on anything below.*

- **exiftool-rs** — a pure-Rust re-implementation of ExifTool (93 readers, 15 writers when checked), licensed **GPL-3.0-or-later**, with tag tables generated from the Perl source: <https://docs.rs/crate/exiftool-rs/latest>. It is a general metadata *editor*.
- **ExifTool** (Perl) and **mat2** (Python) are the usual references for reading and cleaning.
- **Why EXIF-CURAT is different:** MIT licence, specification-only provenance (no generated tables from other tools), and a narrower goal — *audit and sanitise* before sharing a file, with a JSON audit report and fuzzed parsers. It is the native sibling of the browser app EXIF-CURAT on chiuta.ro.
