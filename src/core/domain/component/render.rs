use bevy::asset::Handle;
use bevy::image::Image;

pub struct Renderable {
    texture: Handle<Image>,
}
