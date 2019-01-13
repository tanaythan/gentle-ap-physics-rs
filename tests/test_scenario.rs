extern crate gentle_ap_physics_rs;

use gentle_ap_physics_rs::entity::BaseEntity;
use gentle_ap_physics_rs::entity::worldstate::WorldState;
use gentle_ap_physics_rs::entity::sphere::Sphere;
use gentle_ap_physics_rs::entity::plane::Plane;
use gentle_ap_physics_rs::util::vector3::Vector3;
use std::collections::HashMap;

#[test]
fn test_scenario_collision() {
    let state = Vector3::new(1.0, 1.0, 1.0);
    let init_pos_1 = Vector3::new(2.0, 2.0, 2.0);
    let init_pos_2 = Vector3::new(5.0, 5.0, 5.0);
    let v = Vector3::new(1.0, 1.0, 1.0);
    let m = 1.0;
    let r = 1.0;
    let mut sphere1 = Sphere::new(init_pos_1, m, r, v);
    let mut sphere2 = Sphere::new(init_pos_2, m, r, v);
    let plane = Plane::new(state, 1.0, 2.0, 4.0);
    let mut all_entities: HashMap<String, Box<BaseEntity>> = HashMap::new();
    let plane1_key = String::from("Plane1");
    let sphere1_key = String::from("Sphere1");
    let sphere2_key = String::from("Sphere2");
    all_entities.insert(plane1_key, Box::new(plane));
    all_entities.insert(sphere1_key, Box::new(sphere1));
    all_entities.insert(sphere2_key, Box::new(sphere2));
    let mut state = WorldState::new(all_entities);
    let dt = 1.0;
    
    // Should not collide before starting updates
    assert_eq!(false, sphere1.is_collided(sphere2));
    
    // After updates should collide
    state.update_entities(dt);
    state.update_entities(dt);
    state.update_entities(dt);
    assert_eq!(true, sphere1.is_collided(sphere2));
}