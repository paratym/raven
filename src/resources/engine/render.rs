use pyrite::{
    app::AppBuilder,
    vulkan::{self, QueueConfig, QueueResolution, Vulkan, VulkanConfig},
    window::{Window, WindowConfig},
};
use winit::event_loop::EventLoop;

use crate::constants;

pub fn window(app_builder: &mut AppBuilder, event_loop: &EventLoop<()>) {
    app_builder.add_resource(Window::new(
        &WindowConfig {
            title: constants::WINDOW_NAME.to_string(),
        },
        event_loop,
    ));
}

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
                        vulkan::QueueCapability::Graphics,
                        vulkan::QueueCapability::Compute,
                        vulkan::QueueCapability::Transfer,
                        vulkan::QueueCapability::Present,
                    ],
                    priority: 1.0,
                    resolution: QueueResolution::Panic,
                },
            ],
            enable_validation: true,
            swapchain_support: vulkan::SwapchainSupport::Supported(&*window, &*window),
        })
    });
}
