PNG chunks: strip `tEXt`, `iTXt`, `zTXt`, `eXIf`, `tIME`; keep what affects rendering
good first issue,area:png

New module `png.rs` with `scan`/`strip`; CRC recomputation not needed because chunks are removed whole.

See `CONTRIBUTING.md`. Cite the public source you follow in the PR and in `CLEANROOM.md`.
