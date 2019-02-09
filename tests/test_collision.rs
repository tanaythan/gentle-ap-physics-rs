extern crate gentle_ap_physics_rs;

use gentle_ap_physics_rs::entity::plane::Plane;
use gentle_ap_physics_rs::entity::sphere::Sphere;
use gentle_ap_physics_rs::entity::worldstate::WorldState;
use gentle_ap_physics_rs::entity::BaseEntity;
use gentle_ap_physics_rs::entity::Entity;
use gentle_ap_physics_rs::util::vector3::Vector3;
use std::collections::HashMap;

#[test]
fn test_collision() {
    let state = Vector3::new(1.0, 1.0, 1.0);
    let init_pos_1 = Vector3::new(2.0, 2.0, 2.0);
    let init_pos_2 = Vector3::new(5.0, 2.0, 2.0);
    let v1 = Vector3::new(1.0, 0.0, 0.0);
    let v2 = Vector3::new(-1.0, 0.0, 0.0);
    let m = 1.0;
    let r = 1.0;
    let mut sphere1 = Sphere::new(String::from("Sphere1"), init_pos_1, m, r, v1);
    let mut sphere2 = Sphere::new(String::from("Sphere2"), init_pos_2, m, r, v2);
    let plane = Plane::new(String::from("Plane1"), state, 1.0, 2.0, 4.0);
    let plane1_key = String::from("Plane1");
    let sphere1_key = String::from("Sphere1");
    let sphere2_key = String::from("Sphere2");
    let mut state = WorldState::new();
    state.add("Plane1", Entity::Plane(plane));
    state.add("Sphere1", Entity::Sphere(sphere1));
    state.add("Sphere2", Entity::Sphere(sphere2));
    let dt = 1.0;

    // Should not collide before starting updates
    let mut sphere1Box = state.get(String::from("Sphere1"));
    let mut sphere2Box = state.get(String::from("Sphere2"));

    assert_eq!(false, sphere1Box.is_collided(sphere2Box));

    // After updates should collide
    state.step(dt);
    state.step(dt);

    let mut sphere1Box = state.get(String::from("Sphere1"));
    let mut sphere2Box = state.get(String::from("Sphere2"));

    assert_eq!(true, sphere1Box.is_collided(sphere2Box));
}
