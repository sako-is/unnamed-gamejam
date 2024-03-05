use bevy::prelude::*;
use bevy_cursor::prelude::*;

use crate::game::player::*;
use crate::game::animation::*;

const OFFSET: f32 = 1.0;

pub fn player_movement(
    mut player: Query<(&mut Transform, 
                       &mut PlayerController, 
                       &mut TextureAtlas, 
                       &mut Handle<Image>, 
                       &mut Sprite), 
                With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    kbd_input: Res<ButtonInput<KeyCode>>,
    cursor: Res<CursorLocation>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let Ok((mut transform, mut controller, mut atlas, mut image, mut sprite)) = 
                        player.get_single_mut() else { panic!("Controller not found") };
    let Ok(mut camera_transform) = camera.get_single_mut() else {
                                            panic!("Camera not found") };
    
    let old_facing = &controller.facing.clone();
    
    let Some(position) = cursor.position() else { return; };

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
   

    if (position.x > transform.translation.x) 
    && (position.y < transform.translation.y + OFFSET) 
    && (position.y > transform.translation.y - OFFSET) {
        controller.facing = FacingPosition::Back;

    } else if (position.x < transform.translation.x)
    && (position.y < transform.translation.y + OFFSET)
    && (position.y > transform.translation.y - OFFSET) {
        controller.facing = FacingPosition::Front;
    } else if position.y > transform.translation.y + OFFSET {
        controller.facing = FacingPosition::Right;
    } else if position.y < transform.translation.y - OFFSET {
        controller.facing = FacingPosition::Left;
    }
    /* Set the player to show its back 
    * if mouse position is greater than the player position
    */

    if *old_facing != controller.facing {
        return;
    }

    if (*old_facing == FacingPosition::Left || *old_facing == FacingPosition::Right)
        && (controller.facing == FacingPosition::Left 
        || controller.facing == FacingPosition::Right) {
            sprite.flip_x = !sprite.flip_x;
    } else if controller.facing == FacingPosition::Left {
        let animation_indices = AnimationIndices { 
            first: 0, 
            last: 4 
        };
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(64.0, 64.0), 
            5, 1, None, None);
        *image = asset_server.load("character/left_idle.png");
        *atlas = TextureAtlas {
            layout: texture_atlas_layouts.add(layout),
            index: animation_indices.first 
        }
    } else if controller.facing == FacingPosition::Right {
        let animation_indices = AnimationIndices { 
            first: 0, 
            last: 4 
        };
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(64.0, 64.0), 
            5, 1, None, None);
        *image = asset_server.load("character/right_idle.png");
        *atlas = TextureAtlas {
            layout: texture_atlas_layouts.add(layout),
            index: animation_indices.first
        }
    }

    if controller.facing == FacingPosition::Front {
        let animation_indices = AnimationIndices { 
            first: 0, 
            last: 4 
        };
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(64.0, 64.0), 
            5, 1, None, None);
        *image = asset_server.load("character/front_idle.png");
        *atlas = TextureAtlas {
            layout: texture_atlas_layouts.add(layout),
            index: animation_indices.first
        }
    }

    if controller.facing == FacingPosition::Back {
        let animation_indices = AnimationIndices { 
            first: 0, 
            last: 11 
        };
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(64.0, 64.0), 
            12, 1, None, None);
        *image = asset_server.load("character/back.png");
        *atlas = TextureAtlas {
            layout: texture_atlas_layouts.add(layout),
            index: animation_indices.first
        }
    }
}
