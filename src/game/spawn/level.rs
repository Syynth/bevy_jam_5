//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

use crate::game::town::Town;

use super::{
    // player::SpawnPlayer,
    tilemap::SpawnRandomTilemap,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.add_plugins(PanCamPlugin::default());
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    // commands.trigger(SpawnPlayer);
    commands.trigger(SpawnRandomTilemap {
        town: Town::new(45, 45),
    });
}
