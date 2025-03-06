use crate::core::domain::component::render::{AnimationIndices, AnimationTimer};
use crate::core::domain::entity::class::{ClassBundle, ClassType};
use bevy::asset::{AssetServer, Assets};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::{
    Camera2d, Commands, Res, ResMut, Sprite, TextureAtlas, TextureAtlasLayout, Timer, TimerMode,
    Transform,
};

pub fn spawn_user(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("user.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ClassBundle::new(ClassType::Warrior),
    ));
}
