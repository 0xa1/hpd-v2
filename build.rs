use std::{env, fs, io::Read, path::PathBuf};

use const_gen::{const_declaration, CompileConst};
use xz2::read::XzEncoder;

fn main() {
    println!("cargo:rerun-if-changed=keyboard.toml");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=vial.json");

    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    fs::write(out.join("memory.x"), include_bytes!("memory.x")).expect("write linker memory map");
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    let definition = fs::read_to_string("vial.json").expect("read Vial definition");
    let definition = json::parse(&definition).expect("parse Vial definition");
    let compact = json::stringify(definition);
    let mut compressed = Vec::new();
    XzEncoder::new(compact.as_bytes(), 6)
        .read_to_end(&mut compressed)
        .expect("compress Vial definition");

    // Stable HPD v2 UID; hpd-v2.vil stores the same bytes as a little-endian integer.
    let uid = [0x43u8, 0xE7, 0x23, 0xD1, 0xE2, 0x52, 0xAE, 0x46];
    let constants = [
        const_declaration!(pub VIAL_KEYBOARD_DEF = compressed),
        const_declaration!(pub VIAL_KEYBOARD_ID = uid.to_vec()),
    ];
    fs::write(out.join("config_generated.rs"), constants.join("\n"))
        .expect("write Vial definition");
}
