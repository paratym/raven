use pyrite::app::schedule::ScheduleBuilder;
use pyrite::app::AppBuilder;

mod engine;
mod game;

pub fn setup_raven_schedule(app_builder: &mut AppBuilder) {
    let mut schedule = ScheduleBuilder::new();

    // Input is controlled by entry_point
    schedule.add_task(engine::assets::update);
    schedule.add_task(engine::time::update);
    schedule.add_task(game::entity::camera::update);
    schedule.add_task(game::render::pipeline::world::update_descriptor_sets);
    schedule.add_task((
        engine::render::manager::submit,
        game::render::pipeline::world::update_descriptor_sets,
    ));

    app_builder.set_schedule(schedule.build());
    // window state cleared after all systems in event loop
}
