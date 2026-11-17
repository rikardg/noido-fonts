//! Deflates each enabled family's font files into OUT_DIR and writes
//! `faces.rs`: the face table and the licence table.

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::PathBuf;

/// (feature, folder, licence file name)
const FAMILIES: &[(&str, &str, &str)] = &[
    ("libron", "libron", "LICENSE"),
    ("libertinus-serif", "libertinus-serif", "OFL.txt"),
    ("libertinus-sans", "libertinus-sans", "OFL.txt"),
    ("newcm", "newcm", "License.txt"),
    ("monaspace-neon", "monaspace-neon", "LICENSE"),
    ("alegreya-sans", "alegreya-sans", "OFL.txt"),
];

fn main() {
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut faces = String::new();
    let mut licences = String::new();

    for &(feature, folder, licence) in FAMILIES {
        let var = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"));
        if std::env::var_os(var).is_none() {
            continue;
        }
        let dir = root.join("fonts").join(folder);
        println!("cargo:rerun-if-changed={}", dir.display());
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| matches!(p.extension().and_then(|e| e.to_str()), Some("ttf" | "otf")))
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no fonts in {}", dir.display());

        let mut family_name = None;
        for path in files {
            let data = std::fs::read(&path).unwrap();
            let face = ttf_parser::Face::parse(&data, 0).unwrap();
            let family = typographic_family(&face);
            match &family_name {
                None => family_name = Some(family.clone()),
                Some(f) => assert_eq!(f, &family, "mixed families in {}", dir.display()),
            }
            let name = path.file_name().unwrap().to_str().unwrap();
            std::fs::write(out.join(format!("{name}.z")), deflate(&data)).unwrap();
            writeln!(
                faces,
                "    Face {{ family: {family:?}, italic: {}, weight: {}, deflated: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{name}.z\")) }},",
                face.is_italic(),
                face.weight().to_number(),
            )
            .unwrap();
        }
        writeln!(
            licences,
            "    Licence {{ family: {:?}, text: include_str!({:?}) }},",
            family_name.unwrap(),
            dir.join(licence).to_str().unwrap(),
        )
        .unwrap();
    }

    let src = format!(
        "static FACES: &[Face] = &[\n{faces}];\nstatic LICENCES: &[Licence] = &[\n{licences}];\n"
    );
    std::fs::write(out.join("faces.rs"), src).unwrap();
}

/// nameID 16 (typographic family), else 1 (family).
fn typographic_family(face: &ttf_parser::Face) -> String {
    let find = |id: u16| {
        face.names()
            .into_iter()
            .filter(|n| n.name_id == id && n.is_unicode())
            .find_map(|n| n.to_string())
    };
    find(ttf_parser::name_id::TYPOGRAPHIC_FAMILY)
        .or_else(|| find(ttf_parser::name_id::FAMILY))
        .expect("font has no family name")
}

fn deflate(data: &[u8]) -> Vec<u8> {
    let mut enc = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
    enc.write_all(data).unwrap();
    enc.finish().unwrap()
}
