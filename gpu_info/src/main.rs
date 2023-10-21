use gpu_info::{Nvml, NvmlClockTypeT};

fn main() {
    let nvml = Nvml::default();
    nvml.init_nvml().unwrap();
    let result = nvml.device_get_count();
    let device_id = nvml.get_handle_by_index(0).unwrap();
    let process_name = nvml.system_get_process_name(2485);
    let device_name = nvml.device_get_name(&device_id);
    let memory_info = nvml.device_get_memory_info(&device_id);
    let power_usage = nvml.device_get_power_management_limit(&device_id);
    let max_clock_info = nvml.device_get_max_clock_info(&device_id, NvmlClockTypeT::Graphics);
    let clock_info = nvml.device_get_clock_info(&device_id, NvmlClockTypeT::Graphics);
    let device_temp = nvml.device_get_temperature(&device_id, 0);
    let device_fan_speed = nvml.device_get_fan_speed(&device_id);
    let device_power_usage = nvml.device_get_power_usage(&device_id);
    let device_pcie_throughput = nvml.device_get_pcie_throughput(&device_id, 0);
    let device_utilization_rates = nvml.device_get_utilization_rates(&device_id);
    let device_compute_processes = nvml.device_get_compute_running_processes(&device_id);
    let device_graphics_processes = nvml.device_get_graphics_running_processes(&device_id);
    nvml.shutdown_nvml().unwrap();

    println!("result: {:?}", result);
    println!("&device_id: {:?}", &device_id);
    println!("process_name: {:?}", process_name);
    println!("device_name: {:?}", device_name);
    println!("memory_info: {:?}", memory_info);
    println!("power_usage: {:?}", power_usage);
    println!("max_clock_info: {:?}", max_clock_info);
    println!("clock_info: {:?}", clock_info);
    println!("device_temp: {:?}", device_temp);
    println!("device_fan_speed: {:?}", device_fan_speed);
    println!("device_power_usage: {:?}", device_power_usage);
    println!("device_pcie_throughput: {:?}", device_pcie_throughput);
    println!("device_utilization_rates: {:?}", device_utilization_rates);
    println!("device_graphics_processes: {:?}", device_graphics_processes);
    println!("device_compute_processes: {:?}", device_compute_processes);

    for process in device_graphics_processes.unwrap().iter() {
        println!("process: {:?}", process);
    }
}
