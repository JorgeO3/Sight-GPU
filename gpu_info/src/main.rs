use gpu_info::Nvml;

fn main() {
    let nvml = Nvml;
    nvml.init_nvml().unwrap();
    let result = nvml.device_get_count();
    let device_id = nvml.get_handle_by_index(0);
    let process_name = nvml.system_get_process_name(2485);
    nvml.shutdown_nvml().unwrap();
    println!("{:?}", result);
    println!("{:?}", device_id);
    println!("{:?}", process_name);
}
