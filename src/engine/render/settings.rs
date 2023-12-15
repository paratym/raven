use pyrite::{app::resource::Resource, vulkan::util::Extent2D};

#[derive(Resource)]
pub struct RenderSettings {
    pub render_extent: Extent2D,
}
