use pyrite::app::schedule::ScheduleBuilder;
use pyrite::app::system::SystemFunctionHandler;
use pyrite::app::AppBuilder;

mod engine;
mod game;

pub fn setup_raven_schedule(app_builder: &mut AppBuilder) {
    let mut schedule = ScheduleBuilder::new();

    schedule.add_task(engine::assets::update);
    schedule.add_task(engine::time::update);
    schedule.add_task(game::camera::update);
    schedule.add_task((game::player::update, game::camera::update));
    schedule.add_task((engine::render::manager::submit, game::player::update));

    app_builder.set_schedule(schedule.build());
    // window state cleared after all systems in event loop
}
