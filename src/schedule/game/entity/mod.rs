use pyrite::{
    app::resource::{Res, ResMut},
    input::Input,
};
pub mod camera {
    use crate::game::entity::camera::Camera;

    pub use super::*;

    pub fn update(mut camera: ResMut<Camera>, input: Res<Input>) {
        camera.update(&*input);
    }
}
