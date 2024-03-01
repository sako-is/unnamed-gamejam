use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;

mod camera;
mod game;

fn main() {
    println!("Hello World");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .insert_resource(ClearColor(Color::hex("#87ceeb").unwrap()))
        .add_systems(Startup, game::spawn_lights)
        .add_systems(Startup, camera::setup_camera)
        .add_systems(Startup, game::character)
        .run();
}

