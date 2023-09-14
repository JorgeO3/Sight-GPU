mod bindings;

use bindings::*;
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
    fn from(err: &NvmlReturnT) -> Self {
        match *err {
            NvmlReturnT::NvmlSuccess => NvmlError::NvmlSuccess,
            NvmlReturnT::NvmlErrorUninitialized => NvmlError::NVMLERRORUNINITIALIZED,
            NvmlReturnT::NvmlErrorInvalidArgument => NvmlError::NVMLERRORINVALIDARGUMENT,
            NvmlReturnT::NvmlErrorNotSupported => NvmlError::NVMLERRORNOTSUPPORTED,
            NvmlReturnT::NvmlErrorNoPermission => NvmlError::NVMLERRORNOPERMISSION,
            NvmlReturnT::NvmlErrorAlreadyInitialized => NvmlError::NVMLERRORALREADYINITIALIZED,
            NvmlReturnT::NvmlErrorNotFound => NvmlError::NVMLERRORNOTFOUND,
            NvmlReturnT::NvmlErrorInsufficientSize => NvmlError::NVMLERRORINSUFFICIENTSIZE,
            NvmlReturnT::NvmlErrorInsufficientPower => NvmlError::NVMLERRORINSUFFICIENTPOWER,
            NvmlReturnT::NvmlErrorDriverNotLoaded => NvmlError::NVMLERRORDRIVERNOTLOADED,
            NvmlReturnT::NvmlErrorTimeout => NvmlError::NVMLERRORTIMEOUT,
            NvmlReturnT::NvmlErrorIrqIssue => NvmlError::NVMLERRORIRQISSUE,
            NvmlReturnT::NvmlErrorLibraryNotFound => NvmlError::NVMLERRORLIBRARYNOTFOUND,
            NvmlReturnT::NvmlErrorFunctionNotFound => NvmlError::NVMLERRORFUNCTIONNOTFOUND,
            NvmlReturnT::NvmlErrorCorruptedInforom => NvmlError::NVMLERRORCORRUPTEDINFOROM,
            NvmlReturnT::NvmlErrorGpuIsLost => NvmlError::NVMLERRORGPUISLOST,
            NvmlReturnT::NvmlErrorResetRequired => NvmlError::NVMLERRORRESETREQUIRED,
            NvmlReturnT::NvmlErrorOperatingSystem => NvmlError::NVMLERROROPERATINGSYSTEM,
            NvmlReturnT::NvmlErrorLibRmVersionMismatch => NvmlError::NVMLERRORLIBRMVERSIONMISMATCH,
            NvmlReturnT::NvmlErrorInUse => NvmlError::NVMLERRORINUSE,
            NvmlReturnT::NvmlErrorMemory => NvmlError::NVMLERRORMEMORY,
            NvmlReturnT::NvmlErrorNoData => NvmlError::NVMLERRORNODATA,
            NvmlReturnT::NvmlErrorVgpuEccNotSupported => NvmlError::NVMLERRORVGPUECCNOTSUPPORTED,
            NvmlReturnT::NvmlErrorInsufficientResources => NvmlError::NVMLERRORINSUFFICIENTRESOURCES,
            NvmlReturnT::NvmlErrorFreqNotSupported => NvmlError::NVMLERRORFREQNOTSUPPORTED,
            NvmlReturnT::NvmlErrorArgumentVersionMismatch => NvmlError::NVMLERRORARGUMENTVERSIONMISMATCH,
            NvmlReturnT::NvmlErrorDeprecated => NvmlError::NVMLERRORDEPRECATED,
            NvmlReturnT::NvmlErrorUnknown => NvmlError::NVMLERRORUNKNOWN,
        }
    }
}

fn init_nvml() -> Result<(), NvmlError> {
    let reuslt = unsafe { nvmlInit() };
    todo!();
    match reuslt {
        NvmlReturnT::NvmlSuccess => Ok(()),
        _ => Err(NvmlErrror::from(&result)),
    }
}

fn shutdown_nvml() {}
