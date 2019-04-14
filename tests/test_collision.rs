extern crate gentle_ap_physics_rs;

use gentle_ap_physics_rs::entity::plane::Plane;
use gentle_ap_physics_rs::entity::sphere::Sphere;
use gentle_ap_physics_rs::entity::worldstate::WorldState;
use gentle_ap_physics_rs::entity::BaseEntity;
use gentle_ap_physics_rs::entity::Entity;
use gentle_ap_physics_rs::util::vector3::Vector3;
use gentle_ap_physics_rs::util::math::ACC_GRAVITY;

#[test]
fn test_simple_sphere_collision() {
    let state = Vector3::new(1.0, 1.0, 1.0);
    let init_pos_1 = Vector3::new(2.0, 2.0, 2.0);
    let init_pos_2 = Vector3::new(5.0, 2.0, 2.0);
    let v1 = Vector3::new(1.0, 0.0, 0.0);
    let v2 = Vector3::new(-1.0, 0.0, 0.0);
    let m = 1.0;
    let r = 1.0;
    let plane1_key = String::from("Plane1");
    let sphere1_key = String::from("Sphere1");
    let sphere2_key = String::from("Sphere2");
    let sphere1 = Sphere::new(sphere1_key, init_pos_1, m, r, v1);
    let sphere2 = Sphere::new(sphere2_key, init_pos_2, m, r, v2);
    let plane = Plane::new(plane1_key, state, 1.0, 2.0, 4.0);
    let mut state = WorldState::new();
    state.add("Plane1", Entity::Plane(plane));
    state.add("Sphere1", Entity::Sphere(sphere1));
    state.add("Sphere2", Entity::Sphere(sphere2));
    let dt = 1.0;

    // Should not collide before starting updates
    let mut sphere1_box = state.get(String::from("Sphere1"));
    let mut sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(false, sphere1_box.is_collided(&sphere2_box));

    // After updates should collide
    state.step(dt);

    sphere1_box = state.get(String::from("Sphere1"));
    sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(true, sphere1_box.is_collided(&sphere2_box));

    // After collision acc should change
    state.step(dt);

    sphere1_box = state.get(String::from("Sphere1"));
    sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(false, sphere1_box.is_collided(&sphere2_box));
    assert_eq!(Vector3::new(-1.5, -9.8, 0.0), sphere1_box.get_net_acceleration());
    assert_eq!(Vector3::new(1.5, -9.8, 0.0), sphere2_box.get_net_acceleration());
}

#[test]
fn test_simple_plane_collision() {
    let pos1 = Vector3::new(1.0, 1.0, 1.0);
    let pos2 = Vector3::new(1.0, 1.0, 1.0);
    let plane1 = Plane::new(String::from("Plane1"), pos1, 1.0, 1.0, 1.0);
    let plane2 = Plane::new(String::from("Plane2"), pos2, 1.0, 1.0, 1.0);
    let mut state = WorldState::new();
    state.add("Plane1", Entity::Plane(plane1));
    state.add("Plane2", Entity::Plane(plane2));

    // Planes can not "collide"
    let plane1_box = state.get(String::from("Plane1"));
    let plane2_box = state.get(String::from("Plane2"));
    assert_eq!(false, plane1_box.is_collided(&plane2_box));
}

#[test]
fn test_complex_sphere_collision() {
    let state = Vector3::new(0.0, 0.0, 0.0);
    let init_pos_1 = Vector3::new(0.0, 0.0, 0.0);
    let init_pos_2 = Vector3::new(5.0, 0.0, -5.0);
    let v1 = Vector3::new(1.0, 0.0, -1.0);
    let v2 = Vector3::new(-1.0, 0.0, 1.0);
    let m = 1.0;
    let r = 1.0;
    let sphere1 = Sphere::new(String::from("Sphere1"), init_pos_1, m, r, v1);
    let sphere2 = Sphere::new(String::from("Sphere2"), init_pos_2, m, r, v2);
    let plane = Plane::new(String::from("Plane1"), state, 1.0, 2.0, 4.0);
    let mut state = WorldState::new();
    state.add("Plane1", Entity::Plane(plane));
    state.add("Sphere1", Entity::Sphere(sphere1));
    state.add("Sphere2", Entity::Sphere(sphere2));
    let dt = 1.0;

    // Should not collide before starting updates
    let sphere1_box = state.get(String::from("Sphere1"));
    let sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(false, sphere1_box.is_collided(&sphere2_box));

    // After updates should collide
    state.step(dt);
    state.step(dt);

    let sphere1_box = state.get(String::from("Sphere1"));
    let sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(true, sphere1_box.is_collided(&sphere2_box));

    // After collision acc should change
    state.step(dt);
    state.step(dt);

    let sphere1_box = state.get(String::from("Sphere1"));
    let sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(false, sphere1_box.is_collided(&sphere2_box));
    assert_eq!(Vector3::new(0.0, -9.8, 0.0), sphere1_box.get_net_acceleration());
    assert_eq!(sphere1_box.get_net_acceleration(), sphere2_box.get_net_acceleration());
}

#[test]
fn test_collision_with_additional_forces() {
    let state = Vector3::new(1.0, 1.0, 1.0);
    let init_pos_1 = Vector3::new(2.0, 2.0, 2.0);
    let init_pos_2 = Vector3::new(5.0, 2.0, 2.0);
    let v1 = Vector3::new(1.0, 0.0, 0.0);
    let v2 = Vector3::new(-1.0, 0.0, 0.0);
    let f1_right = Vector3::new(1.0, 0.0, 0.0);
    let f2_right = Vector3::new(1.0, 0.0, 0.0);
    let f1_deep = Vector3::new(0.0, 0.0, -1.0);
    let f2_deep = Vector3::new(0.0, 0.0, -1.0);
    let m = 1.0;
    let r = 1.0;
    let mut sphere1 = Sphere::new(String::from("Sphere1"), init_pos_1, m, r, v1);
    sphere1.apply_force(f1_right);
    sphere1.apply_force(f1_deep);
    let mut sphere2 = Sphere::new(String::from("Sphere2"), init_pos_2, m, r, v2);
    sphere2.apply_force(f2_right);
    sphere2.apply_force(f2_deep);
    let plane = Plane::new(String::from("Plane1"), state, 1.0, 2.0, 4.0);
    let mut state = WorldState::new();
    state.add("Plane1", Entity::Plane(plane));
    state.add("Sphere1", Entity::Sphere(sphere1));
    state.add("Sphere2", Entity::Sphere(sphere2));
    let dt = 1.0;

    // Should not collide before starting updates
    let sphere1_box = state.get(String::from("Sphere1"));
    let sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(false, sphere1_box.is_collided(&sphere2_box));

    // After updates should collide
    state.step(dt);

    let sphere1_box = state.get(String::from("Sphere1"));
    let sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(true, sphere1_box.is_collided(&sphere2_box));

    // After collision acc should change
    state.step(dt);

    let sphere1_box = state.get(String::from("Sphere1"));
    let sphere2_box = state.get(String::from("Sphere2"));

    assert_eq!(false, sphere1_box.is_collided(&sphere2_box));
    assert_eq!(Vector3::new(-0.5, -9.8, -1.0), sphere1_box.get_net_acceleration());
    assert_eq!(Vector3::new(2.5, -9.8, -1.0), sphere2_box.get_net_acceleration());
}
