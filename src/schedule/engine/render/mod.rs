use pyrite::app::{resource::ResMut, AppBuilder};

pub mod manager {
    use pyrite::{
        app::resource::Res,
        vulkan::{swapchain::Swapchain, Vulkan},
    };

    use crate::engine::render::manager::RenderManager;

    pub use super::*;

    pub fn submit(app_builder: &mut AppBuilder) {
        app_builder.add_system(
            |mut render_manager: ResMut<RenderManager>,
             vulkan: Res<Vulkan>,
             swapchain: Res<Swapchain>| {
                render_manager.submit(&*swapchain);
            },
        );
    }
}
