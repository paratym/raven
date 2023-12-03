use pyrite::{app::AppBuilder, asset::Assets, time::Time};

pub mod render;

pub fn assets(app_builder: &mut AppBuilder) {
    app_builder.add_resource(Assets::new());
}

pub fn time(app_builder: &mut AppBuilder) {
    app_builder.add_resource(Time::new());
}
