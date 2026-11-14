# EXIF-CURAT — first ten tasks

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **JPEG robustness: restart markers, fill bytes, progressive files, trailing data** — `good first issue`, `area:jpeg`, `tests`
   Add 10 hand-made edge-case files as byte arrays in tests; none may panic; document what is kept.

2. **PNG chunks: strip `tEXt`, `iTXt`, `zTXt`, `eXIf`, `tIME`; keep what affects rendering** — `good first issue`, `area:png`
   New module `png.rs` with `scan`/`strip`; CRC recomputation not needed because chunks are removed whole.

3. **EXIF reader (TIFF IFD): report GPS, camera, serial and software tags without editing** — `help wanted`, `area:exif`
   Bounds-checked offsets; loop detection; output is a list of findings with human-readable names from the public Exif spec.

4. **XMP packet detection and removal in JPEG, PNG and PDF** — `good first issue`, `area:xmp`
   Detect by the XMP header; report size and the namespaces present.

5. **PDF: Info dictionary and XMP, with the incremental-update caveat** — `help wanted`, `area:pdf`
   Design note first: rewriting vs appending; never report a file as clean if old revisions still hold the data.

6. **OOXML (docx/xlsx/pptx): `docProps`, comments authors, revision ids** — `help wanted`, `area:ooxml`
   ZIP read/write without a GPL dependency; keep the document opening in a word processor (state how you checked).

7. **MP4/MOV: `udta`, `meta`, GPS `©xyz` atoms** — `help wanted`, `area:mp4`
   Atom walker with size checks; strip leaves duration, tracks and codecs intact.

8. **JSON audit report: every finding with offset, kind and a 'would remove' flag** — `good first issue`, `area:cli`
   Stable schema in `docs/AUDIT-SCHEMA.md`; `exif-curat scan --json`.

9. **Fuzz targets with `cargo-fuzz` for every parser, plus a seed corpus** — `help wanted`, `tests`, `area:security`
   CI job that runs each target for 60 s; any crash is a bug report with the input attached.

10. **C2PA / content-credentials: detect and report the manifest, never strip silently** — `help wanted`, `area:provenance`
   Detect JUMBF boxes in JPEG; `strip` keeps them unless `--remove-provenance` is given; explain the trade-off in the docs.
