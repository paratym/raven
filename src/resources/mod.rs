use pyrite::app::AppBuilder;
use winit::event_loop::EventLoop;

mod engine;

pub fn setup_raven_resources(app_builder: &mut AppBuilder, event_loop: &EventLoop<()>) {
    engine::time(app_builder);
    engine::assets(app_builder);
    engine::render::window(app_builder, event_loop)
}
