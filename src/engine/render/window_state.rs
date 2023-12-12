use pyrite::app::resource::Resource;

pub struct WindowResizedEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(Resource)]
pub struct WindowState {
    resized: Option<WindowResizedEvent>,
}

impl WindowState {
    pub fn clear(&mut self) {
        std::mem::take(&mut self.resized);
    }

    pub fn set_resized(&mut self, width: u32, height: u32) {
        self.resized = Some(WindowResizedEvent { width, height });
    }

    pub fn resized(&self) -> Option<&WindowResizedEvent> {
        self.resized.as_ref()
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self { resized: None }
    }
}
