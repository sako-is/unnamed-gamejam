use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use std::vec::Vec;

pub mod generate;

pub struct TileMapPlugin {
	pub x: u16,
	pub y: u16,
	pub tilesize: f32
}

impl Plugin for TileMapPlugin {
	fn build(&self, app: &mut App) {
		app .insert_resource(generate::TileMap {
										x: self.x,
										y: self.y,
										map: Vec::new(), 
										tilesize: self.tilesize
			})
			.add_systems(Startup, spawn_tilemap);
	}
}

pub fn spawn_tilemap(mut commands: Commands, mut tilemap: ResMut<generate::TileMap>, asset_server: Res<AssetServer>) {
	tilemap.map = generate::generate_tilemap(tilemap.x, tilemap.y);
	
	let mut tile: generate::TileType;

	for x in 0..tilemap.x {
		for y in 0..tilemap.y {
			tile = tilemap.map[(y * tilemap.x + x) as usize];

			let tilemap_ent = commands.spawn(SpriteBundle {
				texture: match tile {
					generate::TileType::Floor => asset_server.load("floor.png"),
					generate::TileType::SideWall => asset_server.load("sidewall.png"),
					generate::TileType::FrontWall => asset_server.load("frontwall.png"),
					generate::TileType::TopLeftCorner => asset_server.load("topleft.png")
				}, 
				transform: Transform::from_xyz(
					x as f32 * tilemap.tilesize, 
					-(y as f32 * tilemap.tilesize), 
					0.),
				sprite: Sprite {
					..Default::default()
				},
				..Default::default()
			}).id();

			if tile == generate::TileType::Floor
				|| tile == generate::TileType::SideWall
				|| tile == generate::TileType::FrontWall
				|| tile == generate::TileType::TopLeftCorner {
				commands.entity(tilemap_ent).insert(Collider::cuboid(64., 64.));
			}
		}
	}
}
