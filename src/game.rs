use bevy::prelude::*;

pub mod player;
pub mod movement;
pub mod animation;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app .add_systems(Startup, player::spawn_player)
			.add_systems(Update, movement::player_movement)
			.add_systems(Update, animation::animate_sprite);
	}
}

