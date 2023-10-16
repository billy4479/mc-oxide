fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++20")
        .compile("cxx-demo");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
