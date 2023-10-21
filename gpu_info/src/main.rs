use gpu_info::Nvml;

fn main() {
    let nvml = Nvml::default();
    nvml.init_nvml().unwrap();
    let result = nvml.device_get_count();
    let device_id = nvml.get_handle_by_index(1).unwrap();
    let process_name = nvml.system_get_process_name(2485);
    let device_name = nvml.device_get_name(&device_id);
    nvml.shutdown_nvml().unwrap();

    println!("{:?}", result);
    println!("{:?}", &device_id);
    println!("{:?}", process_name);
    println!("{:?}", device_name);
}
