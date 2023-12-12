use pyrite::app::{resource::ResMut, AppBuilder};

pub mod manager {
    use pyrite::{
        app::resource::Res,
        vulkan::{swapchain::Swapchain, Vulkan},
        window::Window,
    };

    use crate::engine::render::manager::RenderManager;

    pub use super::*;

    pub fn submit(
        mut render_manager: ResMut<RenderManager>,
        vulkan: Res<Vulkan>,
        mut swapchain: ResMut<Swapchain>,
        window: Res<Window>,
    ) {
        render_manager.submit(&*vulkan, &mut *swapchain, &*window);
    }
}
