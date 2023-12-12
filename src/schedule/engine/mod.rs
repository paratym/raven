use pyrite::app::{resource::ResMut, AppBuilder};

pub mod render;

pub mod assets {
    use super::*;
    use pyrite::asset::Assets;

    pub fn update(mut assets: ResMut<Assets>) {
        assets.update();
    }
}

pub mod time {
    use super::*;
    use pyrite::time::Time;

    pub fn update(mut time: ResMut<Time>) {
        time.update();
    }
}
