use pyrite::{app::AppBuilder, vulkan::Vulkan};

pub mod vulkan;

use crate::{constants, engine::render::manager::RenderManager};

pub mod window {
    use crate::engine::render::window_state::WindowState;

    pub use super::*;
    use pyrite::window::{Window, WindowConfig};
    use winit::event_loop::EventLoop;

    pub fn window(app_builder: &mut AppBuilder, event_loop: &EventLoop<()>) {
        app_builder.add_resource(Window::new(
            &WindowConfig {
                title: constants::WINDOW_NAME.to_string(),
            },
            event_loop,
        ));
    }

    pub fn state(app_builder: &mut AppBuilder) {
        app_builder.add_resource(WindowState::default());
    }
}

pub fn render_manager(app_builder: &mut AppBuilder) {
    app_builder.add_resource({
        let vulkan = app_builder.get_resource::<Vulkan>();

        RenderManager::new(&*vulkan)
    });
}
