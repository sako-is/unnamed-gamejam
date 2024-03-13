use bevy::prelude::*;

use bevy_pixel_camera::{
    PixelZoom, PixelViewport
};

#[derive(Component)]
struct CameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(170., -120., 0.),
            ..default()
        },
        PixelZoom::Fixed(4),
        PixelViewport,
    ));
}
