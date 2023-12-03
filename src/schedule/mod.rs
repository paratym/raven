use pyrite::app::AppBuilder;

mod engine;

pub fn setup_raven_schedule(app_builder: &mut AppBuilder) {
    engine::time::update(app_builder);
    engine::assets::update(app_builder);
}
