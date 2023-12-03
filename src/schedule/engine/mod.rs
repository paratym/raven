use pyrite::app::{resource::ResMut, AppBuilder};

pub mod assets {
    use super::*;
    use pyrite::asset::Assets;

    pub fn update(app_builder: &mut AppBuilder) {
        app_builder.add_system(|mut assets: ResMut<Assets>| {
            assets.update();
        });
    }
}

pub mod time {
    use super::*;
    use pyrite::time::Time;

    pub fn update(app_builder: &mut AppBuilder) {
        app_builder.add_system(|mut time: ResMut<Time>| {
            time.update();
        });
    }
}
