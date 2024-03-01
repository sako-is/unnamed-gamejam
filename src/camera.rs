use bevy::prelude::*;

use bevy_pixel_camera::{
    PixelZoom, PixelViewport
};

#[derive(Component)]
struct CameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));
}
