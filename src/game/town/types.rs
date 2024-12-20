use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Acres(u8);

impl From<u8> for Acres {
    fn from(value: u8) -> Self {
        Acres(value)
    }
}

impl From<Acres> for u8 {
    fn from(value: Acres) -> Self {
        value.0
    }
}

impl Deref for Acres {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Acres {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TownType {
    Island,
    Mainland,
}

impl TownType {
    pub fn sample_border(&self, pos: TilePos) -> TileTextureIndex {
        match self {
            TownType::Island => TileTextureIndex(1),
            TownType::Mainland => {
                if pos.y < 10 {
                    return TileTextureIndex(1);
                }
                TileTextureIndex(4)
            }
        }
    }
}
