/// Safe wrappers around amd libraries
#[cfg(feature = "amd")]
pub mod amd;

/// Safe wrappers around intel libraries
#[cfg(feature = "intel")]
pub mod intel;

/// Safe wrappers around nvidia libraries
#[cfg(feature = "nvidia")]
pub mod nvidia;

pub struct GpuResources {}

pub trait GpuResourcesTrait {
    fn get_info(&self) -> GpuResources;
}

pub struct NvidiaGpu;
impl GpuResourcesTrait for NvidiaGpu {
    fn get_info(&self) -> GpuResources {
        GpuResources {}
    }
}

pub struct AmdGpu;
impl GpuResourcesTrait for AmdGpu {
    fn get_info(&self) -> GpuResources {
        GpuResources {}
    }
}

pub struct IntelGpu;
impl GpuResourcesTrait for IntelGpu {
    fn get_info(&self) -> GpuResources {
        GpuResources {}
    }
}

pub enum GpuVendor {
    #[cfg(feature = "amd")]
    Amd(AmdGpu),
    #[cfg(feature = "intel")]
    Intel(IntelGpu),
    #[cfg(feature = "nvidia")]
    Nvidia(NvidiaGpu),
}
impl GpuVendor {
    fn get_info(&self) -> GpuResources {
        match self {
            #[cfg(feature = "amd")]
            GpuVendor::Amd(gpu) => gpu.get_info(),
            #[cfg(feature = "intel")]
            GpuVendor::Intel(gpu) => gpu.get_info(),
            #[cfg(feature = "nvidia")]
            GpuVendor::Nvidia(gpu) => gpu.get_info(),
        }
    }
}

pub struct GpuInfo {
    data: Vec<GpuVendor>,
}
impl GpuInfo {
    fn new() -> Self {
        Self {
            data: vec![
                #[cfg(feature = "amd")]
                GpuVendor::Amd(AmdGpu),
                #[cfg(feature = "intel")]
                GpuVendor::Intel(IntelGpu),
                #[cfg(feature = "nvidia")]
                GpuVendor::Nvidia(NvidiaGpu),
            ],
        }
    }
}
