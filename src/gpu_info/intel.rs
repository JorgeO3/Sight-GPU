#![allow(unused)]

use libc::{c_char, c_uint, c_ulonglong};
use thiserror::Error;

#[link(name = "pcieaccess", kind = "dylib")]
extern "C" {}

struct IntelGpu;

impl IntelGpu {
    fn device_get_count(&self) {}
    fn get_handle_by_index(&self) {}
    fn system_get_process_name(&self) {}
    fn device_get_name(&self) {}
    fn device_get_index(&self) {}
    fn device_get_pcie_version(&self) {}
    fn device_get_memory_info(&self) {}
    fn device_get_power_management_limit(&self) {}
    fn device_get_max_clock_info(&self) {}
    fn device_get_clock_info(&self) {}
    fn device_get_temperature(&self) {}
    fn device_get_fan_speed(&self) {}
    fn device_get_power_usage(&self) {}
    fn device_get_pcie_throughput(&self) {}
    fn device_get_utilization_rates(&self) {}
    fn device_get_compute_running_processes(&self) {}
    fn device_get_graphics_running_processes(&self) {}
}
