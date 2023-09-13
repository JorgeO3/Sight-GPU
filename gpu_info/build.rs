extern crate cc;

fn main() {
    // Compila la librería para NVIDIA
    println!("cargo:rerun-if-changed=c_src/nvidia/nvidia.c");
    println!("cargo:rerun-if-changed=src/nvidia.rs");
    cc::Build::new()
        .file("c_src/nvidia/nvidia.c")
        .include("c_src/nvidia")  // Para incluir archivos de cabecera
        .compile("libnvidia.so");

    // // Compila la librería para AMD
    // cc::Build::new()
    //     .file("c_src/amd/amd.c")
    //     .include("c_src/amd")
    //     .compile("libamd.a");

    // // Compila la librería para Intel
    // cc::Build::new()
    //     .file("c_src/intel/intel.c")
    //     .include("c_src/intel")
    //     .compile("libintel.a");
}
