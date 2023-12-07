use pyrite::app::{resource::ResMut, AppBuilder};

pub mod manager {
    use crate::engine::render::manager::RenderManager;

    pub use super::*;

    pub fn submit(app_builder: &mut AppBuilder) {
        app_builder.add_system(|mut render_manager: ResMut<RenderManager>| {
            render_manager.submit();
        });
    }
}
