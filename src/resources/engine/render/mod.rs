use pyrite::{
    app::AppBuilder,
    vulkan::Vulkan,
    window::{Window, WindowConfig},
};
use winit::event_loop::EventLoop;

pub mod vulkan;

use crate::{constants, engine::render::manager::RenderManager};

pub fn window(app_builder: &mut AppBuilder, event_loop: &EventLoop<()>) {
    app_builder.add_resource(Window::new(
        &WindowConfig {
            title: constants::WINDOW_NAME.to_string(),
        },
        event_loop,
    ));
}

pub fn render_manager(app_builder: &mut AppBuilder) {
    app_builder.add_resource({
        let vulkan = app_builder.get_resource::<Vulkan>();

        RenderManager::new(&*vulkan)
    });
}
