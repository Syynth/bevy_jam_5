use bevy::math::IVec2;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

#[derive(Debug)]
pub struct Town {
    pub width: u32,
    pub height: u32,

    town_type: TownType,

    river: Vec<IVec2>,
}

#[derive(Debug)]
pub enum TownType {
    Island,
    Mainland,
}

impl Town {
    pub fn new(width: u32, height: u32, town_type: TownType) -> Self {
        Self {
            width,
            height,
            town_type,
            river: Vec::new(),
        }
    }

    pub fn sample_tile(&self, pos: TilePos) -> TileTextureIndex {
        match self.town_type {
            TownType::Island => {
                if pos.x > 2 && pos.x < self.width - 3 && pos.y > 2 && pos.y < self.height - 3 {
                    TileTextureIndex(0)
                } else {
                    TileTextureIndex(1)
                }
            }
            TownType::Mainland => TileTextureIndex(0),
        }
    }
}
