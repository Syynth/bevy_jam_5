use bevy::math::IVec2;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use rand::prelude::*;

use generate::generate_river;

mod generate;
mod types;
use types::{Acres, TownType};

// #[derive(Debug, Clone, PartialEq)]
// pub enum Town {
//     IslandTown { size: (Acres, Acres) },
//     MainlandTown { size: (Acres, Acres) },
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Town {
    pub width: u32,
    pub height: u32,

    ledge: f32,
    border_size: u32,

    town_type: types::TownType,

    river: Vec<IVec2>,
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
        let border_size = width / 10;
        Self {
            border_size,
            ledge,
            width,
            height,
            town_type,
            river: generate_river(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                border_size.try_into().unwrap(),
                &mut rng,
            ),
        }
    }

    fn distance_to_edge(&self, pos: TilePos) -> f32 {
        let x = if pos.x < self.width / 2 {
            pos.x as f32
        } else {
            (self.width - pos.x) as f32
        };
        let y = if pos.y < self.height / 2 {
            pos.y as f32
        } else {
            (self.height - pos.y - 1) as f32
        };
        x.min(y)
    }

    pub fn sample_tile(&self, pos: TilePos) -> TileTextureIndex {
        if self.river.contains(&IVec2::new(pos.x as i32, pos.y as i32)) {
            return TileTextureIndex(1);
        }
        if self.distance_to_edge(pos) < 10.0 {
            return self.town_type.sample_border(pos);
        }
        if self.distance_to_edge(pos) < 15.0 {
            return TileTextureIndex(6);
        }

        let ledge_index =
            ((self.height - self.border_size * 2) as f32 * self.ledge) as u32 + self.border_size;

        if pos.y < ledge_index {
            return TileTextureIndex(2);
        }

        if pos.y == ledge_index {
            return TileTextureIndex(3);
        }

        return TileTextureIndex(0);
    }
}
