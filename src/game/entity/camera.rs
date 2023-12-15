use nalgebra::{Isometry3, Vector3};
use pyrite::{
    app::resource::Resource,
    input::{keyboard::Key, Input},
};

#[derive(Resource)]
pub struct Camera {
    isometry: Isometry3<f32>,
    speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            isometry: Isometry3::identity(),
            speed: 1.0,
        }
    }

    pub fn update(&mut self, input: &Input) {
        let mut dv = Vector3::<f32>::zeros();
        if input.is_key_down(Key::W) {
            dv += Vector3::z();
        }
        if input.is_key_down(Key::S) {
            dv -= Vector3::z();
        }
        if input.is_key_down(Key::A) {
            dv -= Vector3::x();
        }
        if input.is_key_down(Key::D) {
            dv += Vector3::x();
        }
        if input.is_key_down(Key::Space) {
            dv += Vector3::y();
        }
        if input.is_key_down(Key::LShift) {
            dv -= Vector3::y();
        }
        if dv.magnitude_squared() > 0.0 {
            dv = dv.normalize();
            self.isometry.translation.vector += dv * self.speed;
            println!("Camera: {:?}", self.isometry.translation.vector);
        }
    }
}
