mod bindings;

use bindings::*;
use libc::c_uint;
use thiserror::Error;

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

#[derive(Debug)]
pub struct SafeNvmlDeviceT(NvmlDeviceT);
impl Default for SafeNvmlDeviceT {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

#[derive(Debug)]
pub struct Nvml;
impl Nvml {
    /// Initializes the NVML library.
    ///
    /// This method provides a safe interface to initialize the NVML library. It wraps
    /// the unsafe `nvmlInit` function.
    pub fn init_nvml(&self) -> Result<(), NvmlError> {
        let result = unsafe { nvmlInit() };
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
        let result = unsafe { nvmlDeviceGetCount(&mut count) };
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
        let result = unsafe { nvmlDeviceGetHandleByIndex(index, &mut device) };
        match result {
            NvmlReturnT::Success => Ok(SafeNvmlDeviceT(device)),
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

    /// Retrieves the process name for a given PID.
    ///
    /// This method provides a safe interface to get the name of the process corresponding
    /// to a PID. It wraps the unsafe `nvmlSystemGetProcessName` function.
    pub fn system_get_process_name(&self, pid: u32) -> Result<String, NvmlError> {
        let mut name: [i8; 64] = [0; 64];
        let result = unsafe { nvmlSystemGetProcessName(pid, name.as_mut_ptr(), 64) };
        let name = self.i8_to_string(&name);

        match result {
            NvmlReturnT::Success => Ok(name),
            _ => Err(NvmlError::from(result)),
        }
    }

    fn i8_to_string(&self, s: &[i8]) -> String {
        // TODO: Add pointer validation
        unsafe { std::ffi::CStr::from_ptr(s.as_ptr() as *const _) }
            .to_str()
            .expect("Failed to convert i8 to string")
            .into()
    }
}
