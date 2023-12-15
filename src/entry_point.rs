use pyrite::{input::keyboard, prelude::*, window};
use winit::{
    event::{
        DeviceEvent as WinitDeviceEvent, ElementState, Event as WinitEvent,
        WindowEvent as WinitWindowEvent,
    },
    event_loop::EventLoop,
};

use crate::engine::render::window_state::WindowState;

pub fn setup_raven_entry_point(app_builder: &mut AppBuilder, event_loop: EventLoop<()>) {
    app_builder.set_entry_point(|mut app| {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop
            .run(move |event, window| match event {
                WinitEvent::WindowEvent { event, .. } => match event {
                    WinitWindowEvent::KeyboardInput { event, .. } => {
                        if let winit::keyboard::PhysicalKey::Code(keycode) = event.physical_key {
                            if let Some(key) = window::util::to_pyrite_key(keycode) {
                                match event.state {
                                    winit::event::ElementState::Pressed => app
                                        .get_resource_mut::<Input>()
                                        .keyboard_mut()
                                        .submit_input(keyboard::SubmitInput::Pressed(key)),
                                    winit::event::ElementState::Released => app
                                        .get_resource_mut::<Input>()
                                        .keyboard_mut()
                                        .submit_input(keyboard::SubmitInput::Released(key)),
                                }
                            }
                        }
                    }
                    WinitWindowEvent::CloseRequested => {
                        window.exit();
                    }
                    WinitWindowEvent::Resized(size) => {
                        app.get_resource_mut::<WindowState>()
                            .set_resized(size.width, size.height);
                    }
                    _ => (),
                },
                // Main game loop.
                winit::event::Event::AboutToWait => {
                    app.execute_schedule();

                    app.get_resource_mut::<WindowState>().clear();
                    app.get_resource_mut::<Input>().clear_inputs();
                }
                _ => (),
            })
            .expect("Failed to run event loop.");
        println!("Quitting raven...");
    });
}
