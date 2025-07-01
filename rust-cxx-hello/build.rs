use cxx_build;

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/hello.cpp")
        .std("c++17")
        .compile("rust-cxx-hello");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/hello.cpp");
    println!("cargo:rerun-if-changed=include/hello.h");
    println!("cargo:rerun-if-changed=include/rust-cxx-hello-lib/rust-cxx-hello-lib.h");
    println!("cargo:rerun-if-changed=lib/rust-cxx-hello-lib/rust-cxx-hello-lib.lib");

    println!("cargo:rustc-link-search=native=lib/rust-cxx-hello-lib");
    println!("cargo:rustc-link-lib=static=rust-cxx-hello-lib");
}