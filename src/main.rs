#![allow(dead_code)]
mod entity;
mod util;

use entity::plane::Plane;
use entity::sphere::Sphere;
use entity::Entity;
use std::collections::HashMap;
use util::vector3::Vector3;

/* We can define sample constants here */
const DT: f32 = 0.1;
const MAX_STEPS: i32 = 20;

fn main() {
    // Initialize our sample entities
    let state = Vector3::new(1.0, 0.0, 3.0);
    let init_pos_1 = Vector3::new(2.0, 4.0, 2.0);
    let init_pos_2 = Vector3::new(5.0, 4.0, 2.0);
    let v1 = Vector3::new(1.0, 0.0, 0.0);
    let v2 = Vector3::new(-1.0, 0.0, 0.0);
    let m = 1.0;
    let r = 1.0;
    let plane = Plane::new(String::from("Plane1"), state.clone(), 1.0, 2.0, 4.0);
    let sphere1 = Sphere::new(String::from("Sphere1"), init_pos_1, m, r, v1);
    let sphere2 = Sphere::new(String::from("Sphere2"), init_pos_2, m, r, v2);
    let mut all_entities: HashMap<String, Entity> = HashMap::new();
    all_entities.insert(String::from("Plane1"), Entity::Plane(plane));
    all_entities.insert(String::from("Sphere1"), Entity::Sphere(sphere1));
    all_entities.insert(String::from("Sphere2"), Entity::Sphere(sphere2));

    // Initialize sample world state
    let mut state = entity::worldstate::WorldState::new_with_map(all_entities);

    let mut i = 1;
    while i <= MAX_STEPS {
        println!("-----------------STEP {:?}-----------------", i);
        state.step(DT);
        i += 1;
    }
}
