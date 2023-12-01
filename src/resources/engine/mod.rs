use pyrite::{app::AppBuilder, asset::Assets};

pub fn assets(app_builder: &mut AppBuilder) {
    app_builder.add_resource(Assets::new());
}
