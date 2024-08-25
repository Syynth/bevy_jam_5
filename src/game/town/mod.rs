use bevy::math::IVec2;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use rand::prelude::*;

use generate::generate_river;

mod generate;

#[derive(Debug)]
pub struct Town {
    pub width: u32,
    pub height: u32,

    ledge: f32,
    border_size: u8,

    town_type: TownType,

    river: Vec<IVec2>,
}

#[derive(Debug)]
pub enum TownType {
    Island,
    Mainland,
}

impl TownType {
    fn sample_border(&self, pos: TilePos) -> TileTextureIndex {
        match self {
            TownType::Island => TileTextureIndex(1),
            TownType::Mainland => {
                if pos.y < 2 {
                    return TileTextureIndex(1);
                }
                TileTextureIndex(4)
            }
        }
    }
}

impl Town {
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = thread_rng();
        let town_type = if rng.gen_bool(0.5) {
            TownType::Island
        } else {
            TownType::Mainland
        };
        let ledge = rng.gen_range(0.25..0.75);
        Self {
            border_size: 2,
            ledge,
            width,
            height,
            town_type,
            river: generate_river(width, height, &mut rng),
        }
    }

    fn is_sample_in_border(&self, pos: TilePos) -> bool {
        pos.x < self.border_size as u32
            || pos.x >= self.width - self.border_size as u32
            || pos.y < self.border_size as u32
            || pos.y >= self.height - self.border_size as u32
    }

    pub fn sample_tile(&self, pos: TilePos) -> TileTextureIndex {
        if self.river.contains(&IVec2::new(pos.x as i32, pos.y as i32)) {
            return TileTextureIndex(1);
        }
        if self.is_sample_in_border(pos) {
            return self.town_type.sample_border(pos);
        }

        let ledge_index = (self.height as f32 * self.ledge) as u32;

        if pos.y < ledge_index {
            return TileTextureIndex(2);
        }

        if pos.y == ledge_index {
            return TileTextureIndex(3);
        }

        return TileTextureIndex(0);
    }
}
