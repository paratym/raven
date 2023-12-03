use pyrite::{app::stage::DEFAULT_STAGE, prelude::*};
use winit::event_loop::EventLoop;

pub fn setup_raven_entry_point(mut app_builder: &mut AppBuilder, event_loop: EventLoop<()>) {
    app_builder.set_entry_point(|mut app| {
        // Make sure we keep polling regardless of whether the window gets new events so we can
        // keep rendering.
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        event_loop
            .run(move |event, window| match event {
                // Capture window events.
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window.exit();
                    }
                    _ => (),
                },
                // Main game loop.
                winit::event::Event::AboutToWait => {
                    app.execute_stage(DEFAULT_STAGE);
                }
                _ => (),
            })
            .expect("Failed to run event loop.");
        println!("Quitting raven...");
    });
}
