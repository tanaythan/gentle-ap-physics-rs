mod entity;
mod util;

use std::collections::HashMap;
use util::vector3::Vector3;

/* We can define physics constants here */
const dt: f32 = 0.1;

fn main() {
    let mut current_time = std::time::Instant::now();
    let mut accumulator = 0.0;

    //Initialize our planes and spheres
    let state = Vector3::new(1.0, 2.0, 3.0);
    let plane = entity::plane::Plane::new(state.clone(), 1.0, 2.0, 4.0);
    let mut all_entities: HashMap<String, Box<entity::BaseEntity>> = HashMap::new();
    println!("{:?}", plane);
    all_entities.insert(String::from("Plane1"), Box::new(plane));

    //Initialize world states
    let mut prev = entity::worldstate::WorldState::new(all_entities.clone());
    let mut curr = entity::worldstate::WorldState::new(all_entities);

    loop {
        let new_time = std::time::Instant::now();
        let mut frame_time = duration_to_s(new_time.duration_since(current_time)); // from ns to s
        if frame_time > 25 {
            // where did this constant come from?
            frame_time = 25;
        }
        current_time = new_time;

        accumulator += frame_time as f32;

        while accumulator >= dt {
            prev = curr.clone();
            curr = curr.update_entities(dt);
            accumulator -= dt;
        }

        // Need to implement add overload for world state

        let alpha = accumulator / dt;
        let lerp_state: entity::worldstate::WorldState =
            curr.clone() * alpha + prev.clone() * (1.0 - alpha);
        lerp_state.print_state();
    }
}

fn duration_to_s(duration: std::time::Duration) -> u64 {
    let nanos = duration.subsec_nanos() as u64;
    let s = (1000 * 1000 * 1000 * duration.as_secs() + nanos) / (1000 * 1000 * 1000);
    s
}
