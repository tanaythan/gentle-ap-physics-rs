#![allow(dead_code)]
mod entity;
mod util;
mod gfx;

extern crate rayon_hash;
extern crate three;
extern crate ears;

use std::{thread, time};
use entity::plane::Plane;
use entity::sphere::Sphere;
use entity::Entity;
use gfx::Renderer;
use rayon_hash::hash_map::HashMap;
use util::vector3::Vector3;
use ears::{Sound, AudioController};

/* We can define sample constants here */
const DT: f32 = 0.1;
const MAX_STEPS: i32 = 10020;

fn main() {
    // Initialize our sample entities
    let state = Vector3::new(1.0, 0.0, 3.0);
    let init_pos_1 = Vector3::new(1.0, 3.0, 3.0);
    let init_pos_2 = Vector3::new(1.0, 4.0, 3.0);
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
    let mut renderer = Renderer::new();

    // Initialize sample world state
    let mut state = entity::worldstate::WorldState::new_with_map(all_entities);

    let mut i = 1;
    let mut is_open = true;

    thread::spawn(|| {
        play_sick_beats();
    });

    while i <= MAX_STEPS && is_open {
        println!("-----------------STEP {:?}-----------------", i);
        state.step(DT);
        is_open = renderer.render();
        i += 1;
        thread::sleep(time::Duration::from_millis(1000));
    }

}



fn play_sick_beats() {
    loop {
        let mut snd = Sound::new("./assets/sick_beats.wav").unwrap();
        snd.play();
        while snd.is_playing() {}
    }
}
