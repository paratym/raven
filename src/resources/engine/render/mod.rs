use pyrite::{
    app::AppBuilder,
    vulkan::{util::Extent2D, Vulkan},
};

pub mod vulkan;

use crate::{
    constants,
    engine::render::{
        manager::{FrameIndex, RenderManager},
        settings::RenderSettings,
    },
};

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

pub fn frame_index(app_builder: &mut AppBuilder) {
    app_builder.add_resource(FrameIndex::new());
}

pub fn render_manager(app_builder: &mut AppBuilder) {
    app_builder.add_resource({
        let vulkan = app_builder.get_resource::<Vulkan>();

        RenderManager::new(&*vulkan)
    });
}

pub fn settings(app_builder: &mut AppBuilder) {
    app_builder.add_resource(RenderSettings {
        render_extent: Extent2D {
            width: 1920,
            height: 1080,
        },
    });
}
