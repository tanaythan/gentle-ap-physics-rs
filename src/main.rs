#![allow(dead_code)]
mod entity;
mod gfx;
mod util;

extern crate ears;
extern crate rayon_hash;
extern crate three;

use ears::{AudioController, Sound};
use entity::plane::Plane;
use entity::sphere::Sphere;
use entity::Entity;
use gfx::Renderer;
use rayon_hash::hash_map::HashMap;
use std::{thread, time};
use util::vector3::Vector3;

/* We can define sample constants here */
const DT: f32 = 1.0;
const MAX_STEPS: i32 = 10020;
const SLEEP_MS: u64 = 1000;

fn main() {
    let mut renderer = Renderer::new();

    // Initialize our sample entities
    let plane_pos = Vector3::new(0.0, 0.0, 0.0);
    let sphere1_pos = Vector3::new(0.0, 10.0, 0.0);
    let sphere1_force = Vector3::new(0.0, 0.0, 0.0);
    let m = 1.0;
    let r = 1.0;

    let plane = Plane::new(String::from("Plane1"), plane_pos, 1.0, 10.0, 10.0);
    let sphere1 = Sphere::new(String::from("Sphere1"), sphere1_pos, m, r, sphere1_force);
    let mut all_entities: HashMap<String, Entity> = HashMap::new();

    all_entities.insert(String::from("Plane1"), Entity::Plane(plane));
    all_entities.insert(String::from("Sphere1"), Entity::Sphere(sphere1));

    renderer.set_graphics(all_entities.get_mut(&String::from("Plane1")).unwrap());
    renderer.set_graphics(all_entities.get_mut(&String::from("Sphere1")).unwrap());

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
        is_open = renderer.render(&state.all_entities());
        i += 1;
        thread::sleep(time::Duration::from_millis(SLEEP_MS));
    }
}

fn play_sick_beats() {
    loop {
        let mut snd = Sound::new("./assets/sick_beats.wav").unwrap();
        snd.play();
        while snd.is_playing() {}
    }
}

fn render(
    window: &mut three::Window,
    camera: &three::camera::Camera,
    entities: &rayon_hash::hash_map::HashMap<String, Entity>,
) -> bool {
    let is_updated = window.update();
    if is_updated {
        for (_key, ent) in entities {
            ent.render();
        }
        window.render(camera);
    }
    is_updated
}
