use pyrite::{app::stage::DEFAULT_STAGE, prelude::*};
use winit::event_loop::EventLoop;

use crate::engine::render::window_state::WindowState;

pub fn setup_raven_entry_point(app_builder: &mut AppBuilder, event_loop: EventLoop<()>) {
    app_builder.set_entry_point(|mut app| {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop
            .run(move |event, window| match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window.exit();
                    }
                    winit::event::WindowEvent::Resized(size) => {
                        app.get_resource_mut::<WindowState>()
                            .set_resized(size.width, size.height);
                    }
                    _ => (),
                },
                // Main game loop.
                winit::event::Event::AboutToWait => {
                    app.execute_schedule();
                    app.get_resource_mut::<WindowState>().clear();
                }
                _ => (),
            })
            .expect("Failed to run event loop.");
        println!("Quitting raven...");
    });
}
