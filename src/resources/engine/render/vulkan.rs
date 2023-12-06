use pyrite::{
    app::AppBuilder,
    vulkan::{
        self,
        swapchain::{Swapchain, SwapchainCreateInfo},
        QueueCapability, QueueConfig, QueuePriority, QueueResolution, SwapchainSupport, Vulkan,
        VulkanConfig,
    },
    window::Window,
};

use crate::constants;

pub fn vulkan(app_builder: &mut AppBuilder) {
    app_builder.add_resource({
        let window = app_builder.get_resource::<Window>();

        Vulkan::new(&VulkanConfig {
            app_name: constants::APP_NAME.to_string(),
            queues: vec![
                // Ensure we have the default queue set.
                QueueConfig {
                    name: vulkan::DEFAULT_QUEUE.to_string(),
                    capabilities: vec![
                        QueueCapability::Graphics,
                        QueueCapability::Compute,
                        QueueCapability::Transfer,
                        QueueCapability::Present,
                    ],
                    priority: QueuePriority::Exclusive,
                    resolution: QueueResolution::Panic,
                },
                QueueConfig {
                    name: "staging".to_string(),
                    capabilities: vec![QueueCapability::Transfer],
                    priority: QueuePriority::Exclusive,
                    resolution: QueueResolution::Panic,
                },
                QueueConfig {
                    name: "staging2".to_string(),
                    capabilities: vec![QueueCapability::Transfer],
                    priority: QueuePriority::Exclusive,
                    resolution: QueueResolution::Panic,
                },
                QueueConfig {
                    name: "staging3".to_string(),
                    capabilities: vec![QueueCapability::Transfer],
                    priority: QueuePriority::Exclusive,
                    resolution: QueueResolution::Panic,
                },
                QueueConfig {
                    name: "staging4".to_string(),
                    capabilities: vec![QueueCapability::Transfer],
                    priority: QueuePriority::Exclusive,
                    resolution: QueueResolution::Panic,
                },
            ],
            enable_validation: true,
            swapchain_support: SwapchainSupport::Supported(&*window, &*window),
        })
    });
}

pub fn swapchain(app_builder: &mut AppBuilder) {
    app_builder.add_resource({
        let vulkan = app_builder.get_resource::<Vulkan>();
        let window = app_builder.get_resource::<Window>();

        let mut swapchain = Swapchain::new();
        swapchain.refresh(
            &*vulkan,
            &SwapchainCreateInfo {
                width: window.width(),
                height: window.height(),
                present_mode: ash::vk::PresentModeKHR::FIFO,
            },
        );

        swapchain
    });
}
