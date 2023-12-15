use pyrite::{
    app::resource::Resource,
    vulkan::{
        objects::{DescriptorSetHandle, DescriptorSetLayout, DescriptorSetPool},
        Vulkan,
    },
};

use crate::{constants, game::entity::camera::Camera};

pub struct Frame {
    frame_descriptor_set: DescriptorSetHandle,
}

#[derive(Resource)]
pub struct WorldRenderPipeline {
    descriptor_pool: DescriptorSetPool,
    frame_descriptor_set_layout: DescriptorSetLayout,
    frames: [Frame; constants::FRAMES_IN_FLIGHT],
}

impl WorldRenderPipeline {
    pub fn new(vulkan: &Vulkan) -> Self {
        let mut descriptor_pool = DescriptorSetPool::new(vulkan);

        let frame_descriptor_set_layout = {
            let builder = DescriptorSetLayout::builder();

            builder.build(vulkan)
        };

        let frames = descriptor_pool
            .allocate_descriptor_sets::<{ constants::FRAMES_IN_FLIGHT }>(
                &frame_descriptor_set_layout,
            )
            .map(|frame_descriptor_set| Frame {
                frame_descriptor_set,
            });

        Self {
            descriptor_pool,
            frame_descriptor_set_layout,
            frames,
        }
    }

    pub fn update_descriptor_sets(&mut self, camera: &Camera, frame_index: usize) {
        let frame = &mut self.frames[frame_index];
    }
}
