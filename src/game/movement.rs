use bevy::prelude::*;

use crate::game::player::*;
use crate::game::animation::*;

const OFFSET: f32 = 0.5;

pub fn player_movement(
    mut player: Query<(&mut Transform, 
                       &mut PlayerController, 
                       &mut TextureAtlas, 
                       &mut Handle<Image>,
                       &mut Sprite,
                       &mut AnimationIndices), 
                With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    kbd_input: Res<ButtonInput<KeyCode>>,
    mut cursor_evr: EventReader<CursorMoved>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>

) {
    let Ok((mut transform, mut controller, mut atlas, mut image, mut sprite, mut indices)) = 
                        player.get_single_mut() else { panic!("Controller not found") };
    let Ok(mut camera_transform) = camera.get_single_mut() else {
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
   
    for ev in cursor_evr.read() {
        if (ev.position.y > transform.translation.y) 
        && (ev.position.x < transform.translation.x + OFFSET) 
        && (ev.position.x > transform.translation.x - OFFSET) {
            controller.facing = FacingPosition::Back;
        } else if (ev.position.y < transform.translation.y)
        && (ev.position.x < transform.translation.x + OFFSET * (ev.position.y - transform.translation.y))
        && (ev.position.x > transform.translation.x - OFFSET * (transform.translation.y - ev.position.y)) {
            controller.facing = FacingPosition::Front;
        } else if ev.position.x > transform.translation.x + OFFSET * (ev.position.y - transform.translation.y){
            controller.facing = FacingPosition::Right;
        } else if ev.position.x < transform.translation.x - OFFSET * (transform.translation.y - ev.position.y){
            controller.facing = FacingPosition::Left;
        }
    }

    /* Set the player to show its back 
    * if mouse ev.position is greater than the player ev.position
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
    mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>
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
