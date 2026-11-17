//! Bundled fonts for noido's typesetter. One cargo feature per family;
//! each face is stored zlib-deflated (done in `build.rs`).

/// One font file, zlib-deflated.
#[derive(Debug, Clone, Copy)]
pub struct Face {
    /// The font's own typographic family name.
    pub family: &'static str,
    pub italic: bool,
    /// OS/2 weight class: 400, 700, ...
    pub weight: u16,
    /// zlib (flate2 `ZlibEncoder`) of the font file.
    pub deflated: &'static [u8],
}

/// The licence text of one family.
#[derive(Debug, Clone, Copy)]
pub struct Licence {
    pub family: &'static str,
    pub text: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/faces.rs"));

/// The faces of the enabled families.
pub fn faces() -> &'static [Face] {
    FACES
}

/// The licences of the enabled families.
pub fn licences() -> &'static [Licence] {
    LICENCES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "libron")]
    fn inflate(data: &[u8]) -> Vec<u8> {
        use std::io::Read;
        let mut out = Vec::new();
        flate2::read::ZlibDecoder::new(data)
            .read_to_end(&mut out)
            .unwrap();
        out
    }

    #[test]
    #[cfg(feature = "libron")]
    fn libron_has_four_faces() {
        let f: Vec<_> = faces().iter().filter(|f| f.family == "Libron").collect();
        assert_eq!(f.len(), 4);
        assert!(f.iter().any(|f| f.italic && f.weight == 700));
    }

    #[test]
    #[cfg(feature = "libron")]
    fn a_face_inflates_to_a_font() {
        let face = faces().iter().find(|f| f.family == "Libron").unwrap();
        let bytes = inflate(face.deflated);
        assert!(bytes.starts_with(&[0, 1, 0, 0]) || bytes.starts_with(b"OTTO"));
    }

    #[test]
    #[cfg(not(any(
        feature = "libron",
        feature = "libertinus-serif",
        feature = "libertinus-sans",
        feature = "newcm",
        feature = "monaspace-neon",
        feature = "alegreya-sans"
    )))]
    fn without_features_there_are_no_faces() {
        assert!(faces().is_empty());
        assert!(licences().is_empty());
    }

    #[test]
    fn every_family_has_its_licence() {
        for face in faces() {
            assert!(licences().iter().any(|l| l.family == face.family));
        }
        for l in licences() {
            assert!(!l.text.is_empty());
        }
    }

    #[test]
    #[cfg(all(
        feature = "libron",
        feature = "libertinus-serif",
        feature = "libertinus-sans",
        feature = "newcm",
        feature = "monaspace-neon",
        feature = "alegreya-sans"
    ))]
    fn all_families_present() {
        let count = |name: &str| faces().iter().filter(|f| f.family == name).count();
        assert_eq!(faces().len(), 4 + 4 + 3 + 4 + 4 + 3);
        assert_eq!(count("Libertinus Serif"), 4);
        assert_eq!(count("Libertinus Sans"), 3);
        assert_eq!(count("Monaspace Neon"), 4);
        assert_eq!(count("Alegreya Sans"), 3);
    }
}
