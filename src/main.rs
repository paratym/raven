use pyrite::{desktop::window::WindowState, prelude::*};

pub mod constants;
pub mod engine;
pub mod game;

mod entry_point;
mod resources;
mod schedule;

fn main() {
    let mut app_builder = AppBuilder::new();

    // Setup all the resources.
    resources::setup_raven_resources(&mut app_builder);

    // Setup the master schedule.
    schedule::setup_raven_schedule(&mut app_builder);

    // Setup the entry point.
    app_builder.set_entry_point(entry_point::entry_point);

    // Run the application.
    app_builder.run();
}
