use pyrite::app::AppBuilder;

mod engine;

pub fn setup_raven_resources(app_builder: &mut AppBuilder) {
    engine::assets(app_builder);
}
