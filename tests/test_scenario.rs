extern crate gentle_ap_physics_rs;

use gentle_ap_physics_rs::entity;

#[test]
fn test_scenario_1() {
    let state = entity::Vector3::new(1.0, 2.0, 3.0);
    let sphereA = entity::sphere::Sphere::new(state.clone(), 1.0, 2.0, 4.0);
    let sphereB = entity::sphere::Sphere::new(state.clone(), 1.0, 2.0, 4.0);
    let plane = entity::plane::Plane::new(state.clone(), 1.0, 2.0, 4.0);
    let all_entities: Vec<Box<entity::BaseEntity>> = Vec::new();
    all_entities.push(Box::new(plane));
    all_entities.push(Box::new(sphereA));
    all_entities.push(Box::new(sphereB));

    let state = entity::worldstate::WorldState::new(all_entities);
    let mut t = 1.0;
    let dt = 1.0;
    
    state.update_entities(t, dt);
}