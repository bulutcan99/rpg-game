mod error;

use bevy::app::{App, PluginGroup};
use bevy::math::Vec2;
use bevy::prelude::{ImagePlugin, MonitorSelection};
use bevy::window::{Window, WindowLevel, WindowPlugin, WindowPosition};
use bevy::DefaultPlugins;


#[tokio::main]
async fn main() -> error::Result<()> {
	let window_plugin = WindowPlugin {
		primary_window: Some(Window {
			title: "Rpg Game".to_string(),
			name: Some("Rpg Game".to_string()),
			window_level: WindowLevel::AlwaysOnTop,
			position: WindowPosition::Centered(MonitorSelection::Primary),
			resolution: Vec2::new(512.0, 512.0).into(),
			..Default::default()
		}),
		..Default::default()
	};

	App::new().add_plugins(DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest())).run();
	Ok(())
}