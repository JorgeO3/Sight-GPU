#![allow(unused)]
use libc::{c_char, c_uint, c_ulonglong, c_void};

/// Maximum length for GPU-related names (e.g., device names).
///
/// This constant defines the upper limit for the length of strings
/// that represent names related to the GPU. This might be used, for example,
/// to allocate buffers for reading GPU device names.
const MAX_NAME_LENGTH: usize = 96;

/// Maximum number of processes that can be tracked for GPU usage.
///
/// When querying for processes that utilize the GPU, this constant
/// provides an upper limit on the number of processes that can be
/// reported in a single query. Useful for allocating buffer space
/// or setting boundaries on loops that iterate over processes.
const MAX_PROCESS_COUNT: usize = 100;

/// Maximum length for a PCI bus ID, version 2.
///
/// This constant defines the buffer size required to store
/// a PCI bus ID string in its version 2 format. This can be important
/// for compatibility and interoperability concerns.
const NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE: usize = 16;

/// Maximum length for a PCI bus ID.
///
/// Similar to the version 2 constant, this constant defines the
/// buffer size needed to store a PCI bus ID string. It's used
/// in scenarios where the specific version of the PCI bus ID format
/// isn't known in advance.
const NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE: usize = 32;

/// Rust representation of the NVML type `nvmlDevice_t`.
// pub type NvmlDeviceT = *mut c_void;

#[repr(C)]
pub struct NvmlDeviceOpaque {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type NvmlDeviceT = *mut NvmlDeviceOpaque;

/// Structure representing the memory information of a GPU device.
///
/// This structure provides details about the total, free, and used memory
/// in bytes on a specific GPU. This allows monitoring and management tasks
/// to gauge memory utilization and availability.
#[repr(C)]
pub struct NvmlMemoryT {
    /// Total installed memory in bytes.
    total: c_ulonglong,

    /// Total free memory in bytes.
    free: c_ulonglong,

    /// Total memory currently in use in bytes.
    used: c_ulonglong,
}

/// Structure representing the utilization rates of a GPU device.
///
/// This structure provides percentage values indicating how much of the GPU's
/// capabilities are being used. This includes both the GPU's core and its memory.
/// It provides a snapshot of the current load on the GPU.
#[repr(C)]
pub struct NvmlUtilizationT {
    /// GPU core utilization rate as a percentage.
    gpu: c_uint,

    /// GPU memory utilization rate as a percentage.
    memory: c_uint,
}

/// Structure representing information about a process utilizing a GPU device.
///
/// This structure gives details about a specific process that is running
/// computations on a GPU. This includes the process's ID, how much GPU memory
/// it's using, and other related details. This is useful for monitoring and
/// managing individual processes' GPU usage.
#[repr(C)]
pub struct NvmlProcessInfoT {
    /// Process ID of the application.
    pid: c_uint,

    /// Amount of GPU memory in bytes used by the process.
    used_gpu_memory: c_ulonglong,

    /// GPU instance ID associated with this process.
    gpu_instance_id: c_uint,

    /// Compute instance ID associated with this process.
    compute_instance_id: c_uint,
}

/// Enumeration representing the return codes from NVML.
///
/// Each variant corresponds to a specific status or error code that might be returned
/// by the NVML functions. These can be used to determine the success or failure
/// of an operation and understand the nature of any issues.
#[repr(C)]
#[derive(Debug)]
pub enum NvmlReturnT {
    /// The operation was successful.
    Success = 0,

    /// NVML has not been initialized.
    ErrorUninitialized = 1,

    /// One or more of the arguments passed to the function are invalid.
    ErrorInvalidArgument = 2,

    /// The platform does not support this operation.
    ErrorNotSupported = 3,

    /// The current user does not have permission for this operation.
    ErrorNoPermission = 4,

    /// NVML has already been initialized.
    ErrorAlreadyInitialized = 5,

    /// The requested item was not found.
    ErrorNotFound = 6,

    /// The provided buffer is not large enough.
    ErrorInsufficientSize = 7,

    /// The GPU does not have enough power for the operation.
    ErrorInsufficientPower = 8,

    /// The NVIDIA driver is not loaded.
    ErrorDriverNotLoaded = 9,

    /// The function call timed out.
    ErrorTimeout = 10,

    /// There was an IRQ issue during the function call.
    ErrorIrqIssue = 11,

    /// NVML shared library couldn't be found or loaded.
    ErrorLibraryNotFound = 12,

    /// A required function in the library could not be found.
    ErrorFunctionNotFound = 13,

    /// GPU's infoROM is corrupted.
    ErrorCorruptedInforom = 14,

    /// GPU has fallen off the bus or is otherwise inaccessible.
    ErrorGpuIsLost = 15,

    /// GPU requires a reset.
    ErrorResetRequired = 16,

    /// The GPU operation failed due to the operating system.
    ErrorOperatingSystem = 17,

    /// RM (driver) version mismatch with NVML.
    ErrorLibRmVersionMismatch = 18,

    /// The GPU is already being used and cannot be used again until it is no longer in use.
    ErrorInUse = 19,

    /// Insufficient memory to complete the operation.
    ErrorMemory = 20,

    /// The operation could not be performed because the information was not available.
    ErrorNoData = 21,

    /// The GPU does not support ECC.
    ErrorVgpuEccNotSupported = 22,

    /// Insufficient resources to perform the operation.
    ErrorInsufficientResources = 23,

    /// This frequency does not support the operation.
    ErrorFreqNotSupported = 24,

    /// The NVML library has a different version than the client.
    ErrorArgumentVersionMismatch = 25,

    /// This function is deprecated and should not be used.
    ErrorDeprecated = 26,

    /// An unknown internal error has occurred.
    ErrorUnknown = 999,
}

// External bindings to the NVIDIA Management Library (NVML).
//
// This block provides Rust-friendly interfaces to the functions exposed by
// the NVML library, allowing for interaction with NVIDIA GPUs and obtaining
// various metrics and information about them.
#[rustfmt::skip]
#[link(name = "nvidia-ml", kind = "dylib")]
extern "C" {
    /// Initializes the NVML library.
    ///
    /// This function should be called once before invoking any other NVML API.
    pub fn nvmlInit() -> NvmlReturnT;

    /// Shuts down the NVML library.
    ///
    /// This function should be called once after finishing with the NVML API to free allocated resources.
    pub fn nvmlShutdown() -> NvmlReturnT;

    /// Retrieves the count of GPU devices.
    ///
    /// Writes the device count into the provided pointer.
    pub fn nvmlDeviceGetCount(deviceCount: *mut c_uint) -> NvmlReturnT;

    /// Retrieves a device handle based on its index.
    ///
    /// This handle can then be used in subsequent NVML calls. 
    pub fn nvmlDeviceGetHandleByIndex(index: c_uint, device: *mut NvmlDeviceT) -> NvmlReturnT;

    /// Retrieves the name of a system process based on its ID.
    ///
    /// Writes the process name into the provided buffer.
    pub fn nvmlSystemGetProcessName(pid: c_uint, name: *mut c_char, lenght: c_uint) -> NvmlReturnT;

    /// Retrieves the name of a GPU device.
    ///
    /// Writes the device name into the provided buffer.
    pub fn nvmlDeviceGetName(device: NvmlDeviceT, name: *mut c_char, lenght: c_uint) -> NvmlReturnT;

    /// Retrieves the index of a GPU device.
    ///
    /// Writes the device index into the provided pointer.
    pub fn nvmlDeviceGetIndex(device: NvmlDeviceT, index: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the maximum PCIe link generation supported by the device.
    ///
    /// Writes the maximum link generation into the provided pointer.
    pub fn nvmlDeviceGetMaxPcieLinkGeneration(device: NvmlDeviceT, maxLinkGen: *mut c_uint) -> NvmlReturnT;

    /// Retrieves memory information for a GPU device.
    ///
    /// Writes the memory information into the provided structure.
    pub fn nvmlDeviceGetMemoryInfo(device: NvmlDeviceT, memory: *mut NvmlMemoryT) -> NvmlReturnT;

    /// Retrieves the power management limit of a GPU device.
    ///
    /// Writes the power limit into the provided pointer.
    pub fn nvmlDeviceGetPowerManagementLimit(device: NvmlDeviceT, limit: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the maximum clock information for a specified type.
    ///
    /// Writes the maximum clock information into the provided pointer.
    pub fn nvmlDeviceGetMaxClockInfo(device: NvmlDeviceT, clockType: c_uint, clockMHz: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the current clock information for a specified type.
    ///
    /// Writes the current clock information into the provided pointer.
    pub fn nvmlDeviceGetClockInfo(device: NvmlDeviceT, clockType: c_uint, clockMHz: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the current temperature of a GPU device for a specified sensor type.
    ///
    /// Writes the temperature into the provided pointer.
    pub fn nvmlDeviceGetTemperature(device: NvmlDeviceT, sensorType: c_uint, temp: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the current fan speed of a GPU device.
    ///
    /// Writes the fan speed into the provided pointer.
    pub fn nvmlDeviceGetFanSpeed(device: NvmlDeviceT, speed: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the current power usage of a GPU device.
    ///
    /// Writes the power usage into the provided pointer.
    pub fn nvmlDeviceGetPowerUsage(device: NvmlDeviceT, power: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the PCIe throughput of a GPU device for a specified counter.
    ///
    /// Writes the PCIe throughput into the provided pointer.
    pub fn nvmlDeviceGetPcieThroughput(device: NvmlDeviceT, counter: c_uint, value: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the current utilization rates (for GPU and memory) of a GPU device.
    ///
    /// Writes the utilization rates into the provided structure.
    pub fn nvmlDeviceGetUtilizationRates(device: NvmlDeviceT, utilization: *mut NvmlUtilizationT) -> NvmlReturnT;

    /// Retrieves information about compute processes running on a GPU device.
    ///
    /// Writes the processes' information into the provided structure.
    pub fn nvmlDeviceGetComputeRunningProcesses(device: NvmlDeviceT, infoCount: *mut c_uint, infos: *mut NvmlProcessInfoT) -> NvmlReturnT;

    /// Retrieves information about graphics processes running on a GPU device.
    ///
    /// Writes the processes' information into the provided structure.
    pub fn nvmlDeviceGetGraphicsRunningProcesses(device: NvmlDeviceT, infoCount: *mut c_uint, infos: *mut NvmlProcessInfoT) -> NvmlReturnT;
}
