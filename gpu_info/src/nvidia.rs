use libc::{c_char, c_uint, c_ulonglong};
use thiserror::Error;

/// Maximum length for GPU-related names (e.g., device names).
///
/// This constant defines the upper limit for the length of strings
/// that represent names related to the GPU. This might be used, for example,
/// to allocate buffers for reading GPU device names.
pub const MAX_NAME_LENGTH: usize = 96;

/// Maximum number of processes that can be tracked for GPU usage.
///
/// When querying for processes that utilize the GPU, this constant
/// provides an upper limit on the number of processes that can be
/// reported in a single query. Useful for allocating buffer space
/// or setting boundaries on loops that iterate over processes.
pub const MAX_PROCESS_COUNT: usize = 100;

/// Maximum length for a PCI bus ID, version 2.
///
/// This constant defines the buffer size required to store
/// a PCI bus ID string in its version 2 format. This can be important
/// for compatibility and interoperability concerns.
pub const NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE: usize = 16;

/// Maximum length for a PCI bus ID.
///
/// Similar to the version 2 constant, this constant defines the
/// buffer size needed to store a PCI bus ID string. It's used
/// in scenarios where the specific version of the PCI bus ID format
/// isn't known in advance.
pub const NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE: usize = 32;

/// Rust representation of the NVML type `nvmlDevice_t`.
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
#[derive(Debug, Default)]
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
#[derive(Debug, Default)]
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
#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Copy)]
pub struct NvmlProcessInfoT {
    /// Process ID of the application.
    pid: c_uint,

    /// Amount of GPU memory in bytes used by the process.
    usedGpuMemory: c_ulonglong,

    /// GPU instance ID associated with this process.
    gpuInstanceId: c_uint,

    /// Compute instance ID associated with this process.
    computeInstanceId: c_uint,
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
    pub fn nvmlInit_v2() -> NvmlReturnT;

    /// Shuts down the NVML library.
    ///
    /// This function should be called once after finishing with the NVML API to free allocated resources.
    pub fn nvmlShutdown() -> NvmlReturnT;

    /// Retrieves the count of GPU devices.
    ///
    /// Writes the device count into the provided pointer.
    pub fn nvmlDeviceGetCount_v2(device_count: *mut c_uint) -> NvmlReturnT;

    /// Retrieves a device handle based on its index.
    ///
    /// This handle can then be used in subsequent NVML calls. 
    pub fn nvmlDeviceGetHandleByIndex_v2(index: c_uint, device: *mut NvmlDeviceT) -> NvmlReturnT;

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
    pub fn nvmlDeviceGetMaxPcieLinkGeneration(device: NvmlDeviceT, max_link_gen: *mut c_uint) -> NvmlReturnT;

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
    pub fn nvmlDeviceGetMaxClockInfo(device: NvmlDeviceT, clock_type: c_uint, clock_mhz: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the current clock information for a specified type.
    ///
    /// Writes the current clock information into the provided pointer.
    pub fn nvmlDeviceGetClockInfo(device: NvmlDeviceT, clock_type: c_uint, clock_mhz: *mut c_uint) -> NvmlReturnT;

    /// Retrieves the current temperature of a GPU device for a specified sensor type.
    ///
    /// Writes the temperature into the provided pointer.
    pub fn nvmlDeviceGetTemperature(device: NvmlDeviceT, sensor_type: c_uint, temp: *mut c_uint) -> NvmlReturnT;

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
    pub fn nvmlDeviceGetComputeRunningProcesses_v3(device: NvmlDeviceT, info_count: *mut c_uint, infos: *mut NvmlProcessInfoT) -> NvmlReturnT;

    /// Retrieves information about graphics processes running on a GPU device.
    ///
    /// Writes the processes' information into the provided structure.
    pub fn nvmlDeviceGetGraphicsRunningProcesses_v3(device: NvmlDeviceT, info_count: *mut c_uint, infos: *mut NvmlProcessInfoT) -> NvmlReturnT;
}

#[derive(Debug, Error)]
pub enum NvmlError {
    #[error("Operation was successful")]
    NvmlSuccess,

    #[error("NVML was not first initialized with nvmlInit()")]
    NVMLERRORUNINITIALIZED,

    #[error("A supplied argument is invalid")]
    NVMLERRORINVALIDARGUMENT,

    #[error("The requested operation is not available on the target device")]
    NVMLERRORNOTSUPPORTED,

    #[error("Permission issue with the request")]
    NVMLERRORNOPERMISSION,

    #[error("NVML is already initialized")]
    NVMLERRORALREADYINITIALIZED,

    #[error("The requested item was not found")]
    NVMLERRORNOTFOUND,

    #[error("The buffer size was insufficient to hold the result")]
    NVMLERRORINSUFFICIENTSIZE,

    #[error("Device does not have enough power to perform the operation")]
    NVMLERRORINSUFFICIENTPOWER,

    #[error("NVIDIA driver is not loaded")]
    NVMLERRORDRIVERNOTLOADED,

    #[error("The operation timed out")]
    NVMLERRORTIMEOUT,

    #[error("A system interrupt occurred")]
    NVMLERRORIRQISSUE,

    #[error("NVML shared library not found")]
    NVMLERRORLIBRARYNOTFOUND,

    #[error("Function not found in the shared library")]
    NVMLERRORFUNCTIONNOTFOUND,

    #[error("Information ROM corrupted")]
    NVMLERRORCORRUPTEDINFOROM,

    #[error("The GPU has fallen off the bus or undergone a reset")]
    NVMLERRORGPUISLOST,

    #[error("The GPU requires a reset before operations can continue")]
    NVMLERRORRESETREQUIRED,

    #[error("The GPU control device has been blocked by the operating system/cgroups")]
    NVMLERROROPERATINGSYSTEM,

    #[error("RM has detected an NVML/RM version mismatch")]
    NVMLERRORLIBRMVERSIONMISMATCH,

    #[error("The GPU is in use or is not responding")]
    NVMLERRORINUSE,

    #[error("Insufficient memory to complete the operation")]
    NVMLERRORMEMORY,

    #[error("No data is available for the requested operation")]
    NVMLERRORNODATA,

    #[error("The device doesn't support vGPU or ECC mode")]
    NVMLERRORVGPUECCNOTSUPPORTED,

    #[error("Operation could not be performed due to insufficient GPU resources")]
    NVMLERRORINSUFFICIENTRESOURCES,

    #[error("Requested clock frequency is not supported")]
    NVMLERRORFREQNOTSUPPORTED,

    #[error("Function has been deprecated")]
    NVMLERRORDEPRECATED,

    #[error("Mismatch arguments passed to function")]
    NVMLERRORARGUMENTVERSIONMISMATCH,

    #[error("An internal driver error occurred")]
    NVMLERRORUNKNOWN,
}

impl From<NvmlReturnT> for NvmlError {
    #[rustfmt::skip]
    fn from(err: NvmlReturnT) -> Self {
        match err {
            NvmlReturnT::Success => NvmlError::NvmlSuccess,
            NvmlReturnT::ErrorUninitialized => NvmlError::NVMLERRORUNINITIALIZED,
            NvmlReturnT::ErrorInvalidArgument => NvmlError::NVMLERRORINVALIDARGUMENT,
            NvmlReturnT::ErrorNotSupported => NvmlError::NVMLERRORNOTSUPPORTED,
            NvmlReturnT::ErrorNoPermission => NvmlError::NVMLERRORNOPERMISSION,
            NvmlReturnT::ErrorAlreadyInitialized => NvmlError::NVMLERRORALREADYINITIALIZED,
            NvmlReturnT::ErrorNotFound => NvmlError::NVMLERRORNOTFOUND,
            NvmlReturnT::ErrorInsufficientSize => NvmlError::NVMLERRORINSUFFICIENTSIZE,
            NvmlReturnT::ErrorInsufficientPower => NvmlError::NVMLERRORINSUFFICIENTPOWER,
            NvmlReturnT::ErrorDriverNotLoaded => NvmlError::NVMLERRORDRIVERNOTLOADED,
            NvmlReturnT::ErrorTimeout => NvmlError::NVMLERRORTIMEOUT,
            NvmlReturnT::ErrorIrqIssue => NvmlError::NVMLERRORIRQISSUE,
            NvmlReturnT::ErrorLibraryNotFound => NvmlError::NVMLERRORLIBRARYNOTFOUND,
            NvmlReturnT::ErrorFunctionNotFound => NvmlError::NVMLERRORFUNCTIONNOTFOUND,
            NvmlReturnT::ErrorCorruptedInforom => NvmlError::NVMLERRORCORRUPTEDINFOROM,
            NvmlReturnT::ErrorGpuIsLost => NvmlError::NVMLERRORGPUISLOST,
            NvmlReturnT::ErrorResetRequired => NvmlError::NVMLERRORRESETREQUIRED,
            NvmlReturnT::ErrorOperatingSystem => NvmlError::NVMLERROROPERATINGSYSTEM,
            NvmlReturnT::ErrorLibRmVersionMismatch => NvmlError::NVMLERRORLIBRMVERSIONMISMATCH,
            NvmlReturnT::ErrorInUse => NvmlError::NVMLERRORINUSE,
            NvmlReturnT::ErrorMemory => NvmlError::NVMLERRORMEMORY,
            NvmlReturnT::ErrorNoData => NvmlError::NVMLERRORNODATA,
            NvmlReturnT::ErrorVgpuEccNotSupported => NvmlError::NVMLERRORVGPUECCNOTSUPPORTED,
            NvmlReturnT::ErrorInsufficientResources => NvmlError::NVMLERRORINSUFFICIENTRESOURCES,
            NvmlReturnT::ErrorFreqNotSupported => NvmlError::NVMLERRORFREQNOTSUPPORTED,
            NvmlReturnT::ErrorArgumentVersionMismatch => NvmlError::NVMLERRORARGUMENTVERSIONMISMATCH,
            NvmlReturnT::ErrorDeprecated => NvmlError::NVMLERRORDEPRECATED,
            NvmlReturnT::ErrorUnknown => NvmlError::NVMLERRORUNKNOWN,
        }
    }
}

/// Enumeration representing the types of sensors available on a GPU device.
///
/// Each variant corresponds to a specific sensor type that can be queried
/// for a GPU device. These can be used to determine which sensors are available
/// and to query for specific sensor information.
pub enum NvmlClockTypeT {
    /// Graphics clock domain.
    Graphics = 0,

    /// SM clock domain.
    SM = 1,

    /// Memory clock domain.
    Memory = 2,

    /// Video encoder/decoder clock domain.
    Video = 3,

    /// Count of clock types.
    Count = 4,
}

/// Safe wrapper for the NVML type `nvmlDevice_t`.
#[derive(Debug)]
pub struct SafeNvmlDeviceT(NvmlDeviceT);
impl Default for SafeNvmlDeviceT {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

#[derive(Debug, Default)]
pub struct Nvml;
impl Nvml {
    fn i8_to_string(&self, s: &[i8]) -> String {
        unsafe { std::ffi::CStr::from_ptr(s.as_ptr() as *const _) }
            .to_str()
            .expect("Failed to convert i8 to string")
            .into()
    }

    /// Initializes the NVML library.
    ///
    /// This method provides a safe interface to initialize the NVML library. It wraps
    /// the unsafe `nvmlInit` function.
    pub fn init_nvml(&self) -> Result<(), NvmlError> {
        let result = unsafe { nvmlInit_v2() };
        match result {
            NvmlReturnT::Success => Ok(()),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Shuts down the NVML library.
    ///
    /// This method provides a safe interface to shut down the NVML library. It wraps
    /// the unsafe `nvmlShutdown` function.
    pub fn shutdown_nvml(&self) -> Result<(), NvmlError> {
        let result = unsafe { nvmlShutdown() };
        match result {
            NvmlReturnT::Success => Ok(()),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the number of devices.
    ///
    /// This method provides a safe interface to get the number of devices. It wraps
    /// the unsafe `nvmlDeviceGetCount` function.
    pub fn device_get_count(&self) -> Result<u32, NvmlError> {
        let mut count: c_uint = 0;
        let result = unsafe { nvmlDeviceGetCount_v2(&mut count) };
        match result {
            NvmlReturnT::Success => Ok(count),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves a handle for a device by its index.
    ///
    /// This method provides a safe interface to get a device handle by its index. It wraps
    /// the unsafe `nvmlDeviceGetHandleByIndex` function.
    pub fn get_handle_by_index(&self, index: u32) -> Result<SafeNvmlDeviceT, NvmlError> {
        let mut device: NvmlDeviceT = std::ptr::null_mut();
        let result = unsafe { nvmlDeviceGetHandleByIndex_v2(index, &mut device) };
        match result {
            NvmlReturnT::Success => Ok(SafeNvmlDeviceT(device)),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the process name for a given PID.
    ///
    /// This method provides a safe interface to get the name of the process corresponding
    /// to a PID. It wraps the unsafe `nvmlSystemGetProcessName` function.
    pub fn system_get_process_name(&self, pid: u32) -> Result<String, NvmlError> {
        let mut name: [i8; MAX_NAME_LENGTH] = [0; MAX_NAME_LENGTH];
        let result = unsafe { nvmlSystemGetProcessName(pid, name.as_mut_ptr(), 64) };
        let name = self.i8_to_string(&name);

        match result {
            NvmlReturnT::Success => Ok(name),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the name of a given device.
    ///
    /// This method provides a safe interface to obtain the name of a specific device
    /// represented by the `SafeNvmlDeviceT` handle. The method wraps the unsafe
    /// `nvmlDeviceGetName` function from the NVML library.
    pub fn device_get_name(&self, device: &SafeNvmlDeviceT) -> Result<String, NvmlError> {
        let mut name: [i8; MAX_NAME_LENGTH] = [0; MAX_NAME_LENGTH];
        let result = unsafe { nvmlDeviceGetName(device.0, name.as_mut_ptr(), 64) };
        let name = self.i8_to_string(&name);

        match result {
            NvmlReturnT::Success => Ok(name),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the index of a device.
    ///
    /// This method provides a safe interface to get the index of a device. It wraps
    /// the unsafe `nvmlDeviceGetIndex` function.
    pub fn device_get_index(&self, device: &SafeNvmlDeviceT) -> Result<u32, NvmlError> {
        let mut index: c_uint = 0;
        let result = unsafe { nvmlDeviceGetIndex(device.0, &mut index) };
        match result {
            NvmlReturnT::Success => Ok(index),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the maximum PCIe generation supported by the device.
    ///
    /// This method provides a safe interface to get the maximum PCIe generation the device supports.
    /// It wraps the unsafe `nvmlDeviceGetMaxPcieLinkGeneration` function.
    pub fn device_get_pcie_version(&self, device: &SafeNvmlDeviceT) -> Result<u32, NvmlError> {
        let mut max_link_gen: c_uint = 0;
        let result = unsafe { nvmlDeviceGetMaxPcieLinkGeneration(device.0, &mut max_link_gen) };
        match result {
            NvmlReturnT::Success => Ok(max_link_gen),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the memory information of a device.
    ///
    /// This method provides a safe interface to obtain detailed memory-related information
    /// for the specified device. It wraps the unsafe `nvmlDeviceGetMemoryInfo` function.
    pub fn device_get_memory_info(&self, device: &SafeNvmlDeviceT) -> Result<NvmlMemoryT, NvmlError> {
        let mut memory: NvmlMemoryT = NvmlMemoryT::default();
        let result = unsafe { nvmlDeviceGetMemoryInfo(device.0, &mut memory) };
        match result {
            NvmlReturnT::Success => Ok(memory),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the power management limit of a device in milli volts.
    ///
    /// This method provides a safe interface to obtain the power management limit of a device.
    /// It wraps the unsafe `nvmlDeviceGetPowerManagementLimit` function.
    pub fn device_get_power_management_limit(&self, device: &SafeNvmlDeviceT) -> Result<u32, NvmlError> {
        let mut limit: c_uint = 0;
        let result = unsafe { nvmlDeviceGetPowerManagementLimit(device.0, &mut limit) };
        match result {
            NvmlReturnT::Success => Ok(limit),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the current clock information for a specified type.
    ///
    /// This method provides a safe interface to obtain the current clock information for a specified type.
    /// It wraps the unsafe `nvmlDeviceGetClockInfo` function.
    pub fn device_get_max_clock_info(
        &self,
        device: &SafeNvmlDeviceT,
        clock_type: NvmlClockTypeT,
    ) -> Result<u32, NvmlError> {
        let mut clock_mhz: c_uint = 0;
        let clock_type = clock_type as c_uint;
        let result = unsafe { nvmlDeviceGetMaxClockInfo(device.0, clock_type, &mut clock_mhz) };
        match result {
            NvmlReturnT::Success => Ok(clock_mhz),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the current clock information for a specified type.
    ///
    /// This method provides a safe interface to obtain the current clock information for a specified type.
    /// It wraps the unsafe `nvmlDeviceGetClockInfo` function.
    pub fn device_get_clock_info(
        &self,
        device: &SafeNvmlDeviceT,
        clock_type: NvmlClockTypeT,
    ) -> Result<u32, NvmlError> {
        let mut clock_mhz: c_uint = 0;
        let clock_type = clock_type as c_uint;
        let result = unsafe { nvmlDeviceGetClockInfo(device.0, clock_type, &mut clock_mhz) };
        match result {
            NvmlReturnT::Success => Ok(clock_mhz),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the current temperature of a device for a specified sensor type.
    ///
    /// This method provides a safe interface to obtain the current temperature of a device for a specified sensor type.
    /// It wraps the unsafe `nvmlDeviceGetTemperature` function.
    pub fn device_get_temperature(&self, device: &SafeNvmlDeviceT, sensor_type: u32) -> Result<u32, NvmlError> {
        let mut temp: c_uint = 0;
        let result = unsafe { nvmlDeviceGetTemperature(device.0, sensor_type, &mut temp) };
        match result {
            NvmlReturnT::Success => Ok(temp),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the current fan speed of a device.
    ///
    /// This method provides a safe interface to obtain the current fan speed of a device.
    /// It wraps the unsafe `nvmlDeviceGetFanSpeed` function.
    pub fn device_get_fan_speed(&self, device: &SafeNvmlDeviceT) -> Result<u32, NvmlError> {
        let mut speed: c_uint = 0;
        let result = unsafe { nvmlDeviceGetFanSpeed(device.0, &mut speed) };
        match result {
            NvmlReturnT::Success => Ok(speed),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the current power usage of a device.
    ///
    /// This method provides a safe interface to obtain the current power usage of a device.
    /// It wraps the unsafe `nvmlDeviceGetPowerUsage` function.
    pub fn device_get_power_usage(&self, device: &SafeNvmlDeviceT) -> Result<u32, NvmlError> {
        let mut power: c_uint = 0;
        let result = unsafe { nvmlDeviceGetPowerUsage(device.0, &mut power) };
        match result {
            NvmlReturnT::Success => Ok(power),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the PCIe throughput of a device for a specified counter.
    ///
    /// This method provides a safe interface to obtain the PCIe throughput of a device for a specified counter.
    /// It wraps the unsafe `nvmlDeviceGetPcieThroughput` function.
    pub fn device_get_pcie_throughput(&self, device: &SafeNvmlDeviceT, counter: u32) -> Result<u32, NvmlError> {
        let mut value: c_uint = 0;
        let result = unsafe { nvmlDeviceGetPcieThroughput(device.0, counter, &mut value) };
        match result {
            NvmlReturnT::Success => Ok(value),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves the current utilization rates of a device.
    ///
    /// This method provides a safe interface to obtain the current utilization rates of a device.
    /// It wraps the unsafe `nvmlDeviceGetUtilizationRates` function.
    pub fn device_get_utilization_rates(&self, device: &SafeNvmlDeviceT) -> Result<NvmlUtilizationT, NvmlError> {
        let mut utilization: NvmlUtilizationT = NvmlUtilizationT::default();
        let result = unsafe { nvmlDeviceGetUtilizationRates(device.0, &mut utilization) };
        match result {
            NvmlReturnT::Success => Ok(utilization),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves information about compute processes running on a device.
    ///
    /// This method provides a safe interface to obtain information about compute processes running on a device.
    /// It wraps the unsafe `nvmlDeviceGetComputeRunningProcesses` function.
    pub fn device_get_compute_running_processes(
        &self,
        device: &SafeNvmlDeviceT,
    ) -> Result<Vec<NvmlProcessInfoT>, NvmlError> {
        let mut infos: Vec<NvmlProcessInfoT> = Vec::with_capacity(MAX_PROCESS_COUNT);
        let mut info_count: c_uint = 10;
        let result = unsafe { nvmlDeviceGetComputeRunningProcesses_v3(device.0, &mut info_count, infos.as_mut_ptr()) };
        match result {
            NvmlReturnT::Success => Ok(infos),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Retrieves information about graphics processes running on a device.
    ///
    /// This method provides a safe interface to obtain information about graphics processes running on a device.
    /// It wraps the unsafe `nvmlDeviceGetGraphicsRunningProcesses` function.
    pub fn device_get_graphics_running_processes(
        &self,
        device: &SafeNvmlDeviceT,
    ) -> Result<Vec<NvmlProcessInfoT>, NvmlError> {
        let mut info_count: c_uint = MAX_PROCESS_COUNT as c_uint;
        let mut infos: [NvmlProcessInfoT; MAX_PROCESS_COUNT] = [NvmlProcessInfoT::default(); MAX_PROCESS_COUNT];
        let result = unsafe { nvmlDeviceGetGraphicsRunningProcesses_v3(device.0, &mut info_count, infos.as_mut_ptr()) };
        println!("info_count: {}", info_count);

        match result {
            NvmlReturnT::Success => Ok(infos[0..=info_count as usize].to_vec()),
            _ => Err(NvmlError::from(result)),
        }
    }
}
