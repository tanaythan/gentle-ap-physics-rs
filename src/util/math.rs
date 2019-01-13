use super::vector3::Vector3;

const COEFFICIENT_OF_FRICTION: f32 = 0.05;
const ACC_GRAVITY: f32 = 9.8;

pub fn friction(f: f32) -> f32 {
    COEFFICIENT_OF_FRICTION * f
}

pub fn gravity(m: f32) -> f32 {
    m * ACC_GRAVITY
}

pub fn velocity(src_pos: Vector3, dst_pos: Vector3, t: f32) -> Vector3 {
    (dst_pos - src_pos) / t
}

pub fn velocity_from_acc(v: Vector3, a: Vector3, t: f32) -> Vector3 {
    v + a * t
}

pub fn new_pos(src_pos: Vector3, v: Vector3, t: f32) -> Vector3 {
    v * t + src_pos
}

fn sqr(x: f32) -> f32 {
    x * x
}

pub fn detect_collide_sphere_to_sphere(src_pos: Vector3, compare_pos: Vector3, 
                                       r1: f32, r2: f32) -> bool {
    sqr(src_pos.x - compare_pos.x) + sqr(src_pos.y - compare_pos.y) + sqr(src_pos.z - compare_pos.z) <= sqr(r1 + r2)
}

pub fn detect_collide_sphere_to_plane(center: f32, plane: f32, 
                                      normal: f32) -> bool {
    (center - plane) * normal <= 0.0
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn it_calc_friction() {
    assert_eq!(0.1, friction(2.0));
  }

  #[test]
  fn it_calc_gravity() {
    assert_eq!(19.6, gravity(2.0));
  }

  #[test]
  fn it_calc_velocity() {
    let src = Vector3::new(1.0, 1.0, 1.0);
    let dst = Vector3::new(2.0, 2.0, 2.0);
    let expected = Vector3::new(1.0, 1.0, 1.0);
    assert_eq!(expected, velocity(src, dst, 1.0));
  }

  #[test]
  fn it_calc_new_pos() {
    let pos = Vector3::new(1.0, 1.0, 1.0);
    let v = Vector3::new(2.0, 2.0, 2.0);
    let expected = Vector3::new(3.0, 3.0, 3.0);
    assert_eq!(expected, new_pos(pos, v, 1.0));
  }

  #[test]
  fn it_calc_detect_collide_sphere_to_sphere() {
    let src = Vector3::new(1.0, 1.0, 1.0);
    let dst = Vector3::new(2.0, 2.0, 2.0);
    assert_eq!(true, detect_collide_sphere_to_sphere(src, dst, 1.0, 1.0));
    assert_eq!(false, detect_collide_sphere_to_sphere(src, dst, 0.0, 0.0));
  }

  #[test]
  fn it_calc_detect_collide_sphere_to_plane() {
    assert_eq!(true, detect_collide_sphere_to_plane(1.0, 1.0, 0.0));
    assert_eq!(false, detect_collide_sphere_to_plane(2.0, 1.0, 1.0));
  }
}