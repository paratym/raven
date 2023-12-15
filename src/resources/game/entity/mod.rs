use pyrite::app::AppBuilder;

use crate::game::entity::camera::Camera;

pub fn camera(app_builder: &mut AppBuilder) {
    app_builder.add_resource(Camera::new());
}
