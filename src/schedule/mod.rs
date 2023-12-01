use pyrite::app::{system::SystemFunctionHandler, AppBuilder};

mod engine;

pub fn setup_raven_schedule(app_builder: &mut AppBuilder) {
    engine::assets::update(app_builder);
}
