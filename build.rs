fn main() {
    #[cfg(feature = "cxx")] {
        cxx_build::bridge("src/lib.rs").compile("yaz0_rust");
    }
    #[cfg(feature = "cabi")] {
        let mut config = cbindgen::Config::default();
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        config.language = cbindgen::Language::C;
        cbindgen::generate_with_config(crate_dir.clone(), config.clone()).unwrap()
        .write_to_file("yaz0_rust.h");
        config.language = cbindgen::Language::Cxx;
        cbindgen::generate_with_config(crate_dir, config).unwrap()
        .write_to_file("yaz0_rust.hh");
    }
}