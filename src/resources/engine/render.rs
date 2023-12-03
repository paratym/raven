use pyrite::{
    app::AppBuilder,
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
