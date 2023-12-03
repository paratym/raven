use pyrite::{
    app::AppBuilder,
    vulkan::{QueueConfig, Vulkan, VulkanConfig},
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
    app_builder.add_resource(Vulkan::new(&VulkanConfig {
        app_name: constants::APP_NAME.to_string(),
        queues: vec![
            // Ensure we have the default queue set.
            QueueConfig {
                name: pyrite::vulkan::DEFAULT_QUEUE.to_string(),
                capabilities: vec![
                    pyrite::vulkan::QueueCapability::Graphics,
                    pyrite::vulkan::QueueCapability::Compute,
                    pyrite::vulkan::QueueCapability::Transfer,
                    pyrite::vulkan::QueueCapability::Present,
                ],
            },
        ],
    }));
}
