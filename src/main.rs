use bevy::prelude::*;
use bevy::window::*;
use bevy_pixel_camera::PixelCameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use rand::*;

mod camera;
mod game;
mod tilemap;

fn main() {
	let mut rng = rand::thread_rng();

	App::new()
		.add_plugins(DefaultPlugins
					.set(ImagePlugin::default_nearest())
					.set(WindowPlugin {
						primary_window: Some(Window {
							resolution: (640., 480.).into(),
							title: "Unnamed Acerola GameJam Game".to_string(),
							present_mode: PresentMode::Fifo,
							resizable: true,
							..default()
						}),
						..default()
					}))
		.insert_resource(ClearColor(Color::hex("#5a6194").unwrap()))
		.add_plugins(tilemap::TileMapPlugin {
							x: rng.gen_range(7..=20),
							y: rng.gen_range(7..=20),
							tilesize: 64.
					})
		.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
		.add_plugins(RapierDebugRenderPlugin::default())
		.add_plugins(PixelCameraPlugin)
		.add_plugins(WorldInspectorPlugin::new())
		.add_systems(Startup, camera::setup_camera)
		.add_plugins(game::GamePlugin)
		.run();
}

