use pyrite::{app::AppBuilder, vulkan::Vulkan};

use crate::game::render::world_pipeline::WorldRenderPipeline;

pub fn world(app_builder: &mut AppBuilder) {
    app_builder.add_resource({
        let vulkan = app_builder.get_resource::<Vulkan>();
        WorldRenderPipeline::new(&*vulkan)
    });
}
