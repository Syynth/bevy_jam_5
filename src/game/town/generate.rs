use bevy::prelude::*;
use rand::prelude::*;

pub fn generate_river(
    town_width: i32,
    town_height: i32,
    min_edge_distance: i32,
    rng: &mut ThreadRng,
) -> Vec<IVec2> {
    let num_turns = rng.gen_range(1..4);
    let mut river = Vec::with_capacity(num_turns + 1);
    let mut last_x = rng.gen_range(min_edge_distance..(town_width - min_edge_distance * 2));
    river.push(IVec2::new(last_x.try_into().unwrap(), 0));
    for turn in min_edge_distance..(town_height - min_edge_distance) {
        let mut should_turn = rng.gen_bool(0.5);
        let turn_dir = rng.gen_range(-1..=1 as i32);

        let y = turn as i32;

        if !should_turn {
            river.push(IVec2::new(last_x.try_into().unwrap(), y));
            continue;
        }

        while should_turn {
            let x = last_x + (turn_dir);
            if x > min_edge_distance && x < (town_width - min_edge_distance * 2).try_into().unwrap()
            {
                last_x = x;
                river.push(IVec2::new(x as i32, y as i32));
                should_turn = rng.gen_bool(0.5);
            } else {
                should_turn = false;
            }
        }
    }
    return river;
}
