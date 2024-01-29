fn main() {
    if let Err(e) = sight_gpu::core::run() {
        eprintln!("e = {:#?}", e);
    }
}
