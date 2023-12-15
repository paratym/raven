use pyrite::app::{resource::ResMut, AppBuilder};

pub mod manager {
    use pyrite::{
        app::resource::Res,
        vulkan::{swapchain::Swapchain, Vulkan},
        window::Window,
    };

    use crate::engine::render::manager::{FrameIndex, RenderManager};

    pub use super::*;

    pub fn submit(
        mut render_manager: ResMut<RenderManager>,
        vulkan: Res<Vulkan>,
        mut swapchain: ResMut<Swapchain>,
        window: Res<Window>,
        mut frame_index: ResMut<FrameIndex>,
    ) {
        render_manager.submit(&*vulkan, &mut *swapchain, &*window, frame_index.get());
        frame_index.increment();
    }
}
