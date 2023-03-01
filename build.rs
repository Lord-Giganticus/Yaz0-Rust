fn main() {
    #[cfg(feature = "cxx")] {
        cxx_build::bridge("src/lib.rs").compile("yaz0_rust");
    }
}