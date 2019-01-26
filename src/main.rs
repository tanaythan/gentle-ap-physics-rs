mod entity;
mod util;

use std::collections::HashMap;
use util::vector3::Vector3;
use util::time::duration_to_s;

/* We can define sample constants here */
const dt: f32 = 0.1;

fn main() {
    //Initialize our sample planes and spheres
    let state = Vector3::new(1.0, 2.0, 3.0);
    let plane = entity::plane::Plane::new(state.clone(), 1.0, 2.0, 4.0);
    let mut all_entities: HashMap<String, Box<entity::BaseEntity>> = HashMap::new();
    all_entities.insert(String::from("Plane1"), Box::new(plane));

    //Initialize sample world state
    let mut state = entity::worldstate::WorldState::new_with_map(all_entities);

    loop {
        state.step(dt)
    }
}
