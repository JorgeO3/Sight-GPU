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
    /// This function wraps around the unsafe `nvmlInit` function and provides a safe interface.
    pub fn init_nvml(&self) -> Result<(), NvmlError> {
        let result = unsafe { nvmlInit() };
        match result {
            NvmlReturnT::Success => Ok(()),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Shuts down the NVML library.
    /// This function wraps around the unsafe `nvmlShutdown` function and provides a safe interface.
    pub fn shutdown_nvml(&self) -> Result<(), NvmlError> {
        let result = unsafe { nvmlShutdown() };
        match result {
            NvmlReturnT::Success => Ok(()),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Get the number of devices
    /// This function wraps around the unsafe `nvmlDeviceGetCount` function and provides a safe interface.
    pub fn device_get_count(&self) -> Result<u32, NvmlError> {
        let mut count: c_uint = 0;
        let result = unsafe { nvmlDeviceGetCount(&mut count) };
        match result {
            NvmlReturnT::Success => Ok(count),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Get the handle of a device
    /// This function wraps around the unsafe `nvmlDeviceGetCount` function and provides a safe interface.
    pub fn get_handle_by_index(&self, index: u32) -> Result<SafeNvmlDeviceT, NvmlError> {
        let mut device: NvmlDeviceT = std::ptr::null_mut();
        let result = unsafe { nvmlDeviceGetHandleByIndex(index, &mut device) };
        match result {
            NvmlReturnT::Success => Ok(SafeNvmlDeviceT(device)),
            _ => Err(NvmlError::from(result)),
        }
    }

    /// Get the index of a device
    /// This function wraps around the unsafe `nvmlDeviceGetCount` function and provides a safe interface.
    pub fn device_get_index(&self, index: u32, device: &SafeNvmlDeviceT) -> Result<u32, NvmlError> {
        let mut index: c_uint = index;
        let result = unsafe { nvmlDeviceGetIndex(device.0, &mut index) };
        match result {
            NvmlReturnT::Success => Ok(index),
            _ => Err(NvmlError::from(result)),
        }
    }
}
