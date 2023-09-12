fn main() {
    // Haz que Cargo recompile tu proyecto si se modifica cualquiera de las bibliotecas de C.
    println!("cargo:rerun-if-changed=path/to/library.c");
    println!("cargo:rerun-if-changed=path/to/headers.h");

    // Enlace con las bibliotecas de C
    println!("cargo:rustc-link-lib=static=name_of_static_lib");
    println!("cargo:rustc-link-lib=dylib=name_of_dynamic_lib");
}
