use bevy::prelude::*;
use rand::prelude::*;

pub fn generate_river(town_width: u32, town_height: u32, rng: &mut ThreadRng) -> Vec<IVec2> {
    let num_turns = rng.gen_range(1..4);
    let mut river = Vec::with_capacity(num_turns + 1);
    let mut last_x = rng.gen_range(2..(town_width - 4) as i32);
    river.push(IVec2::new(last_x, 0));
    for turn in 0..town_height {
        let mut should_turn = rng.gen_bool(0.5);
        let turn_dir = rng.gen_range(-1..=1);

        let y = turn as i32;

        if !should_turn {
            river.push(IVec2::new(last_x, y));
            continue;
        }

        while should_turn {
            let x = last_x + turn_dir;
            if x > 2 && x < (town_width - 3) as i32 {
                last_x = x;
                river.push(IVec2::new(x, y));
                should_turn = rng.gen_bool(0.5);
            } else {
                should_turn = false;
            }
        }
    }
    return river;
}
