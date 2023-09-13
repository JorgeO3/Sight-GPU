#![allow(unused)]

use libc::{c_double, c_uint, c_ulonglong};
use std::os::raw::c_char;

const MAX_NAME_LENGTH: usize = 96;
const MAX_PROCESS_COUNT: usize = 100;
// TODO: ojo con el tipo de estas constantes
// TODO: || || ||
// TODO: \/ \/ \/
const NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE: usize = 16;
const NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE: usize = 32;

// #define NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE   16

// typedef struct nvmlPciInfo_st
// {
//     char busIdLegacy[NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE]; //!< The legacy tuple domain:bus:device.function PCI identifier (&amp; NULL terminator)
//     unsigned int domain;             //!< The PCI domain on which the device's bus resides, 0 to 0xffffffff
//     unsigned int bus;                //!< The bus on which the device resides, 0 to 0xff
//     unsigned int device;             //!< The device's id on the bus, 0 to 31
//     unsigned int pciDeviceId;        //!< The combined 16-bit device id and 16-bit vendor id

//     // Added in NVML 2.285 API
//     unsigned int pciSubSystemId;     //!< The 32-bit Sub System Device ID

//     char busId[NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE]; //!< The tuple domain:bus:device.function PCI identifier (&amp; NULL terminator)
// } nvmlPciInfo_t;

#[repr(C)]
#[derive(Debug)]
struct NvmlPciInfo {
    bus_id_legacy: [c_char; NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE],
    domain: c_uint,
    bus: c_uint,
    device: c_uint,
    pci_device_id: c_uint,
    pci_subsystem_id: c_uint,
    bus_id: [c_char; NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE],
}

// typedef struct nvmlMemory_st
// {
//     unsigned long long total;        //!< Total physical device memory (in bytes)
//     unsigned long long free;         //!< Unallocated device memory (in bytes)
//     unsigned long long used;         //!< Sum of Reserved and Allocated device memory (in bytes).
//                                      //!< Note that the driver/GPU always sets aside a small amount of memory for bookkeeping
// } nvmlMemory_t;
#[repr(C)]
#[derive(Debug)]
struct NvmlMemory {
    total: c_ulonglong,
    free: c_ulonglong,
    used: c_ulonglong,
}

// typedef struct nvmlProcessInfo_st
// {
//     unsigned int        pid;                //!< Process ID
//     unsigned long long  usedGpuMemory;      //!< Amount of used GPU memory in bytes.
//                                             //! Under WDDM, \ref NVML_VALUE_NOT_AVAILABLE is always reported
//                                             //! because Windows KMD manages all the memory and not the NVIDIA driver
//     unsigned int        gpuInstanceId;      //!< If MIG is enabled, stores a valid GPU instance ID. gpuInstanceId is set to
//                                             //  0xFFFFFFFF otherwise.
//     unsigned int        computeInstanceId;  //!< If MIG is enabled, stores a valid compute instance ID. computeInstanceId is set to
//                                             //  0xFFFFFFFF otherwise.
// } nvmlProcessInfo_t;
#[repr(C)]
#[derive(Debug)]
struct NvmlProcessInfo {
    pid: c_uint,
    used_gpu_memory: c_ulonglong,
    gpu_instance_id: c_uint,
    compute_instance_id: c_uint,
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceStaticInfo {
    name: [c_char; MAX_NAME_LENGTH],
    device_id: c_uint,
    pci_info: NvmlPciInfo,
    memory_info: NvmlMemory,
    max_power_consumption: c_uint,
    max_gpu_frequency: c_uint,
    max_memory_frequency: c_uint,
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceDynamicInfo {
    current_gpu_frequency: c_uint,
    current_memory_frequency: c_uint,
    gpu_temperature: c_uint,
    fan_speed_percentage: c_uint,
    current_power_consumption: c_uint,
    rx_bytes_rate: c_ulonglong,
    tx_bytes_rate: c_ulonglong,
    gpu_usage_percentage: c_double,
    used_memory: c_ulonglong,
    processes: [NvmlProcessInfo; MAX_PROCESS_COUNT],
    process_count: c_uint,
}

#[link(name = "nvidia", kind = "dylib")]
#[link(name = "nvidia-ml", kind = "dylib")]
extern "C" {
    pub fn get_device_count() -> c_uint;
    pub fn fetch_all_static_device_info(deviceCount: c_uint) -> DeviceStaticInfo;
    pub fn fetch_all_dynamic_device_info(deviceCount: c_uint) -> DeviceDynamicInfo;
}
