pub const APP_NAME: &str = "raven";
pub const WINDOW_NAME: &str = "Raven";
pub const FRAMES_IN_FLIGHT: usize = 2;

pub const SWAPCHAIN_PREFERRED_IMAGE_COUNT: u32 = (FRAMES_IN_FLIGHT + 1) as u32;
pub const SWAPCHAIN_PREFERRED_PRESENT_MODE: ash::vk::PresentModeKHR = ash::vk::PresentModeKHR::FIFO;

// Re-exports
pub use pyrite::vulkan::DEFAULT_QUEUE;
