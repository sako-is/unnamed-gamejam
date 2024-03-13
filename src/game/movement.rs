use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::player::*;
use crate::game::animation::*;

const OFFSET: f32 = 30.;

pub fn player_movement(
	mut player: Query<(&mut Transform, 
					   &mut PlayerController, 
					   &mut TextureAtlas, 
					   &mut Handle<Image>,
					   &mut Sprite,
					   &mut AnimationIndices), 
				With<Player>>,
	mut q_camera: Query<(&Camera, &mut Transform, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
	kbd_input: Res<ButtonInput<KeyCode>>,
	mut cursor_evr: EventReader<CursorMoved>,
	windows: Query<&Window, With<PrimaryWindow>>,
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>

) {
	let Ok((mut transform, mut controller, mut atlas, mut image, mut sprite, mut indices)) = 
						player.get_single_mut() else { panic!("Controller not found") };
	let Ok((camera, mut camera_transform, camera_global)) = q_camera.get_single_mut() else {
											panic!("Camera not found") };
	
	let old_facing = &controller.facing.clone();
	
	if kbd_input.pressed(KeyCode::KeyW) {
		transform.translation.y += controller.speed * time.delta_seconds();
		camera_transform.translation.y += controller.speed * time.delta_seconds();
		controller.facing = FacingPosition::Back;
	}

	if kbd_input.pressed(KeyCode::KeyS) {
		transform.translation.y -= controller.speed * time.delta_seconds();
		camera_transform.translation.y -= controller.speed * time.delta_seconds();
		controller.facing = FacingPosition::Front;
	}

	if kbd_input.pressed(KeyCode::KeyA) {
		transform.translation.x -= controller.speed * time.delta_seconds();
		camera_transform.translation.x -= controller.speed * time.delta_seconds();
		controller.facing = FacingPosition::Left;
	}
	
	if kbd_input.pressed(KeyCode::KeyD) {
		transform.translation.x += controller.speed * time.delta_seconds();
		camera_transform.translation.x += controller.speed * time.delta_seconds();
		controller.facing = FacingPosition::Right;
	}
  
	for _ in cursor_evr.read() {
		let Some(position) = windows.single().cursor_position()
									.and_then(|cursor| camera.viewport_to_world(camera_global, cursor))
									.map(|ray| ray.origin.truncate()) else { break; };

		if (position.y > transform.translation.y) 
		&& (position.x < transform.translation.x + OFFSET) 
		&& (position.x > transform.translation.x - OFFSET) {
			controller.facing = FacingPosition::Back;
		} else if (position.y < transform.translation.y)
		&& (position.x < transform.translation.x + OFFSET)
		&& (position.x > transform.translation.x - OFFSET) {
			controller.facing = FacingPosition::Front;
		} else if position.x > transform.translation.x + OFFSET {
			controller.facing = FacingPosition::Right;
		} else if position.x < transform.translation.x - OFFSET {
			controller.facing = FacingPosition::Left;
		} 
	}

	/* Set the player to show its back 
	* if mouse position is greater than the player position
	*/
	if *old_facing != controller.facing {
		return;
	}

	if controller.facing == FacingPosition::Left {
		load_side(&mut image, &mut atlas, &mut indices, 
				  &mut sprite, "character/right_idle.png".to_string(), 
				  5, true, &asset_server, &mut texture_atlas_layouts);
	}
	if controller.facing == FacingPosition::Right {
		load_side(&mut image, &mut atlas, &mut indices, 
				  &mut sprite, "character/right_idle.png".to_string(), 
				  5, false, &asset_server, &mut texture_atlas_layouts);
	}
	if controller.facing == FacingPosition::Front {
		load_side(&mut image, &mut atlas, &mut indices, 
				  &mut sprite, "character/front_idle.png".to_string(), 
				  5, false, &asset_server, &mut texture_atlas_layouts);
	}
	if controller.facing == FacingPosition::Back {
		load_side(&mut image, &mut atlas, &mut indices, 
				  &mut sprite, "character/back.png".to_string(), 
				  12, false, &asset_server, &mut texture_atlas_layouts);
	}
}

pub fn load_side(
	image: &mut Handle<Image>, 
	atlas: &mut TextureAtlas, 
	indices: &mut AnimationIndices,
	sprite: &mut Sprite,
	filename: String,
	frames: usize,
	flip: bool,
	asset_server: &Res<AssetServer>,
	texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>
) {
	*image = asset_server.load(filename);
	let layout = TextureAtlasLayout::from_grid(
				Vec2::new(64.0, 64.0), 
				frames, 1, None, None);
	
	*indices = AnimationIndices { 
		first: 0, 
		last: frames - 1
	};

	*atlas = TextureAtlas {
		layout: texture_atlas_layouts.add(layout),
		index: indices.first
	};
	sprite.flip_x = flip;
}
