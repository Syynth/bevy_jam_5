//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;

use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use rand_core::RngCore;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        EntropyPlugin::<WyRand>::default(),
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
    ));
}
