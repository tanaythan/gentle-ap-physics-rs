extern crate gentle_ap_physics_rs;

use gentle_ap_physics_rs::entity::plane::Plane;
use gentle_ap_physics_rs::entity::sphere::Sphere;
use gentle_ap_physics_rs::entity::worldstate::WorldState;
use gentle_ap_physics_rs::entity::BaseEntity;
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
    let mut sphere1 = Sphere::new(init_pos_1, m, r, v1);
    let mut sphere2 = Sphere::new(init_pos_2, m, r, v2);
    let plane = Plane::new(state, 1.0, 2.0, 4.0);
    let mut all_entities: HashMap<String, Box<BaseEntity>> = HashMap::new();
    let plane1_key = String::from("Plane1");
    let sphere1_key = String::from("Sphere1");
    let sphere2_key = String::from("Sphere2");
    all_entities.insert(plane1_key, Box::new(plane));
    all_entities.insert(sphere1_key, Box::new(sphere1));
    all_entities.insert(sphere2_key, Box::new(sphere2));
    let mut state = WorldState::new_with_map(all_entities);
    let dt = 1.0;

    // Should not collide before starting updates
    let mut sphere1Box = state.get(String::from("Sphere1"));
    let mut sphere1Entity: &Sphere = match sphere1Box.as_any().downcast_ref::<Sphere>() {
        Some(b) => b,
        None => panic!("asdasd"),
    };
    let mut sphere2Box = state.get(String::from("Sphere2"));
    let mut sphere2Entity: &Sphere = match sphere2Box.as_any().downcast_ref::<Sphere>() {
        Some(b) => b,
        None => panic!("asdasd"),
    };

    assert_eq!(false, sphere1Entity.is_collided(sphere2Entity));

    // After updates should collide
    state.update_entities(dt);
    state.update_entities(dt);
    state.update_entities(dt);

    let mut sphere1Box = state.get(String::from("Sphere1"));
    let mut sphere1Entity: &Sphere = match sphere1Box.as_any().downcast_ref::<Sphere>() {
        Some(b) => b,
        None => panic!("asdasd"),
    };
    let mut sphere2Box = state.get(String::from("Sphere2"));
    let mut sphere2Entity: &Sphere = match sphere2Box.as_any().downcast_ref::<Sphere>() {
        Some(b) => b,
        None => panic!("asdasd"),
    };

    assert_eq!(true, sphere1Entity.is_collided(sphere2Entity));
}
