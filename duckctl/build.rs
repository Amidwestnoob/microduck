//! The emotions' sounds, embedded: `udp-pad` plays them on the laptop while the duck moves
//! (`src/udp_pad/voice.rs`). Read from the set the robot ships (`robotd/emotions/`), so a wheel
//! built from a checkout carries the sounds of the robots built from it.

use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../robotd/emotions");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut wavs: Vec<_> = std::fs::read_dir(&dir)
        .map(|entries| {
            entries
                .filter_map(|e| Some(e.ok()?.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "wav"))
                .collect()
        })
        .unwrap_or_default();
    wavs.sort();
    let mut out = String::from("pub static EMOTIONS: &[(&str, &[u8])] = &[\n");
    for wav in wavs {
        let name = wav
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("UTF-8 name");
        let path = wav.canonicalize().expect("a readable wav");
        writeln!(out, "    ({name:?}, include_bytes!({:?})),", path.display()).unwrap();
    }
    out.push_str("];\n");
    let target = Path::new(&std::env::var("OUT_DIR").unwrap()).join("emotions.rs");
    std::fs::write(target, out).expect("OUT_DIR is writable");
}
