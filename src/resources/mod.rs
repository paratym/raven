use pyrite::app::AppBuilder;
use winit::event_loop::EventLoop;

mod engine;
mod game;

pub fn setup_raven_resources(app_builder: &mut AppBuilder, event_loop: &EventLoop<()>) {
    engine::input(app_builder);
    engine::time(app_builder);
    engine::assets(app_builder);

    engine::render::window::window(app_builder, event_loop);
    engine::render::window::state(app_builder);

    engine::render::vulkan::vulkan(app_builder);
    engine::render::vulkan::swapchain(app_builder);

    engine::render::settings(app_builder);
    engine::render::frame_index(app_builder);
    engine::render::render_manager(app_builder);

    game::entity::camera(app_builder);

    game::render::pipeline::world(app_builder);
}
