fn main() {
    let root_dir = std::path::Path::new(".");
    let typescript_dir = root_dir.join("typescript").join("src");
    let tsx_dir = root_dir.join("tsx").join("src");
    let common_dir = root_dir.join("common");

    let mut c_config = cc::Build::new();
    c_config.include(&typescript_dir);
    c_config.include(&tsx_dir);
    c_config
        .flag_if_supported("-std=c11")
        .flag_if_supported("-Wno-unused-parameter");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows-msvc") {
        c_config.flag_if_supported("/utf-8");
    }
    if target == "wasm32-unknown-unknown" {
        let Ok(wasm_headers) = std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS") else {
            panic!("Environment variable DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS must be set");
        };
        c_config.include(&wasm_headers);
        c_config.define("NEED_WASM_EXTRA_H", None);

        // Prevent duplicate symbol error in WASM linking.
        c_config.define("NDEBUG", None);
    }

    for path in &[
        typescript_dir.join("parser.c"),
        typescript_dir.join("scanner.c"),
        tsx_dir.join("parser.c"),
        tsx_dir.join("scanner.c"),
    ] {
        c_config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    println!(
        "cargo:rerun-if-changed={}",
        common_dir.join("scanner.h").to_str().unwrap()
    );

    c_config.compile("tree-sitter-typescript");
}
