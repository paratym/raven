use ash::vk;
use pyrite::{vulkan::swapchain::SwapchainCreateInfo, window::Window};

use crate::constants;

pub fn create_swapchain_info(window: &Window) -> SwapchainCreateInfo {
    SwapchainCreateInfo {
        width: window.width(),
        height: window.height(),
        preferred_present_mode: constants::SWAPCHAIN_PREFERRED_PRESENT_MODE,
        preferred_image_count: constants::SWAPCHAIN_PREFERRED_IMAGE_COUNT,
        image_usage: vk::ImageUsageFlags::TRANSFER_DST,
        create_image_views: false,
    }
}
