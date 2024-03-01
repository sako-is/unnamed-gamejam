use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::pbr::{light_consts, CascadeShadowConfigBuilder};
use std::f32::consts::*;

#[derive(Component)]
pub struct Player;

pub fn character(mut commands: Commands, asset_server: Res<AssetServer>,) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("character/back.png"),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    },
    Player));
}

pub fn spawn_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

