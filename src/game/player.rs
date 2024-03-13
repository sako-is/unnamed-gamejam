use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::animation::*;

const IDLE_FRAMES: usize = 12;

#[derive(PartialEq, Clone)]
pub enum FacingPosition {
	Front,
	Back,
	Left,
	Right
}

#[derive(Component)]
pub struct PlayerController {
	pub speed: f32,
	pub facing: FacingPosition
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, 
					asset_server: Res<AssetServer>,
					mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
	let layout = TextureAtlasLayout::from_grid(
				Vec2::new(64.0, 64.0), 
				IDLE_FRAMES, 1, None, None);
	
	let animation_indices = AnimationIndices { 
		first: 0, 
		last: IDLE_FRAMES - 1
	};

	let player = commands.spawn((SpriteSheetBundle {
		texture: asset_server.load("character/back.png"),
		atlas: TextureAtlas {
			layout: texture_atlas_layouts.add(layout),
			index: animation_indices.first
		},
		transform: Transform::from_xyz(170., -120., 1.),
		sprite: Sprite {
			..Default::default()
		},
		..Default::default()
	},
	PlayerController {
		speed: 100.,
		facing: FacingPosition::Back
	},
	animation_indices,
	AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
	Player)).id();

	commands.entity(player)
			.insert(Collider::cuboid(64., 20.))
			.insert(RigidBody::Dynamic);
}


