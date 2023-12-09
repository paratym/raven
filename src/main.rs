use pyrite::prelude::*;
use winit::event_loop::EventLoop;

pub mod constants;
pub mod engine;
pub mod game;

mod entry_point;
mod resources;
mod schedule;

fn main() {
    let mut app_builder = AppBuilder::new();
    let event_loop = EventLoop::new().expect("Failed to create event loop.");

    // Setup all the resources.
    resources::setup_raven_resources(&mut app_builder, &event_loop);

    // Setup the master schedule.
    schedule::setup_raven_schedule(&mut app_builder);

    // Setup the entry point.
    entry_point::setup_raven_entry_point(&mut app_builder, event_loop);

    // Run the application.
    app_builder.run();
}
