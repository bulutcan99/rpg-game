use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::Component;

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
