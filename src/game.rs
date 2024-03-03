use bevy::prelude::*;

pub mod player;
pub mod movement;
pub mod animation;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app .add_systems(Startup, player::spawn_player)
            .add_systems(Startup, spawn_bg)
            .add_systems(Update, movement::player_movement)
            .add_systems(Update, animation::animate_sprite);
    }
}

pub fn spawn_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("floorplan.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        sprite: Sprite {
            ..Default::default()
        },
        ..Default::default()
    });
}

