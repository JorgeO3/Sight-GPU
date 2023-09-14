use gpu_info::nvidia::{nvmlInit, nvmlShutdown, nvmlSystemGetProcessName};
use libc::{c_char, c_uint};

fn main() {
    let pid: c_uint = 2133;
    let mut buffer: [c_char; 256] = [0; 256]; // un buffer de ejemplo con tamaño 256
    let length: c_uint = buffer.len() as c_uint;

    unsafe {
        nvmlInit();
        let data = nvmlSystemGetProcessName(pid, buffer.as_mut_ptr(), length);
        nvmlShutdown();
        println!("{:?}", data);
        let array: [i8; 256] = [0; 256];
        let slice_u8: &[u8] = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, array.len());
        let result = String::from_utf8_lossy(slice_u8);

        for part in result.split('\0') {
            println!("{}", part);
        }
    }
}
