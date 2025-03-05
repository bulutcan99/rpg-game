use bevy::asset::Handle;
use bevy::prelude::{Component, Image, Timer};

#[derive(Component)]
pub struct Renderable {
    pub(crate) texture: Handle<Image>,
}

impl Renderable {
    pub fn new(texture: Handle<Image>) -> Self {
        Self { texture }
    }
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
        }
    }
}

#[derive(Component)]
struct Animation {
    start_frame: usize,
    end_frame: usize,
    current_frame: usize,
    timer: Timer,
}
