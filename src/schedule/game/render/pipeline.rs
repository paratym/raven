pub mod world {
    use pyrite::app::resource::{Res, ResMut};

    use crate::{
        engine::render::manager::FrameIndex,
        game::{entity::camera::Camera, render::world_pipeline::WorldRenderPipeline},
    };

    pub fn update_descriptor_sets(
        mut world_pipeline: ResMut<WorldRenderPipeline>,
        camera: Res<Camera>,
        frame_index: Res<FrameIndex>,
    ) {
        world_pipeline.update_descriptor_sets(&*camera, frame_index.get());
    }
}
