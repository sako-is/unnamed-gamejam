use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use bevy::prelude::*;

#[derive(Clone, PartialEq, Copy)]
pub enum TileType {
    SideWall,
    FrontWall,
    Floor,
    TopLeftCorner
}

impl Distribution<TileType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0..=1) {
            0 => TileType::SideWall,
            1 => TileType::FrontWall,
            _ => TileType::Floor
        }
    }
}

#[derive(Resource)]
pub struct TileMap {
    pub x: u16,
    pub y: u16,
    pub map: Vec<TileType>,
    pub tilesize: f32
}

pub fn generate_tilemap(x: u16, y: u16) -> Vec<TileType> {
    let mut tilemap: Vec<TileType> = vec![TileType::Floor; (x * y).into()];
    let mut done: Vec<u32> = vec![0; (x * y).into()];
    
    let mut rng = rand::thread_rng();

    let mut i: usize;
    for _ in 0..x*y {
        i = rng.gen_range(0..(x*y)) as usize;

        if done[i] == 1 { continue; }

        done[i] = 1;
    }
    
    for j in 0..x*y {
        if j < x {
            tilemap[j as usize] = TileType::FrontWall;
        }

        if j % x == 0 {
            tilemap[j as usize] = TileType::SideWall;
        }
    }
    tilemap[0] = TileType::TopLeftCorner;

    tilemap
}
