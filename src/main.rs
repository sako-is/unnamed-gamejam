use bevy::prelude::*;
use bevy::window::*;
use bevy_pixel_camera::PixelCameraPlugin;
use bevy_cursor::prelude::*;

mod camera;
mod game;

fn main() {
    println!("Hello World");
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
        .add_plugins(TrackCursorPlugin)
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, camera::setup_camera)
        .add_plugins(game::GamePlugin)
        .run();
}

