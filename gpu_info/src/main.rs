use gpu_info::nvidia::{fetch_all_dynamic_device_info, get_device_count};

fn main() {
    unsafe {
        let device_count = get_device_count();
        let dynamic_info = fetch_all_dynamic_device_info(device_count);
        println!("{:?}", dynamic_info);
    }
}
