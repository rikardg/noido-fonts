# noido-fonts

The fonts bundled with noido's typesetter, as a crate. Each face is stored
zlib-deflated (done by `build.rs` at build time); `faces()` lists the faces
of the enabled families and `licences()` their licence texts. Inflate with
flate2's `ZlibDecoder`.

Features (one per family, none on by default):

- `libron` (Libron 0.32)
- `libertinus-serif` (Libertinus Serif 7.051)
- `libertinus-sans` (Libertinus Sans 7.051)
- `newcm` (New Computer Modern 8.1.1, family name `NewComputerModern`)
- `monaspace-neon` (Monaspace Neon 1.400)
- `alegreya-sans` (Alegreya Sans)

Each family's folder under `fonts/` holds its upstream files, its licence
and a `SOURCE.md` with URL, version and file list.
