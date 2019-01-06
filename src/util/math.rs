const COEFFICIENT_OF_FRICTION: f64 = 0.05;
const ACC_GRAVITY: f64 = 9.8;

pub fn friction(f: f64) -> f64 {
    COEFFICIENT_OF_FRICTION * f
}

pub fn gravity(m: f64) -> f64 {
    m * ACC_GRAVITY
}

pub fn velocity(src_pos: f64, dst_pos: f64, t: f64) -> f64 {
    (dst_pos - src_pos) / t
}

pub fn new_pos(src_pos: f64, v: f64, t: f64) -> f64 {
    v * t + src_pos
}

fn sqr(x: f64) -> f64 {
    x * x
}

pub fn detect_collide_sphere_to_sphere(x1: f64, y1: f64, x2: f64, y2: f64, 
                                       r1: f64, r2: f64) -> bool {
    sqr(x1 - x2) + sqr(y1 - y2) <= sqr(r1 + r2)
}

pub fn detect_collide_sphere_to_plane(center: f64, plane: f64, 
                                      normal: f64) -> bool {
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
    assert_eq!(1.0, velocity(1.0, 2.0, 1.0));
  }

  #[test]
  fn it_calc_new_pos() {
    assert_eq!(3.0, new_pos(1.0, 2.0, 1.0));
  }

  #[test]
  fn it_calc_detect_collide_sphere_to_sphere() {
    assert_eq!(true, detect_collide_sphere_to_sphere(1.0, 1.0, 1.0, 1.0, 1.0, 1.0));
    assert_eq!(false, detect_collide_sphere_to_sphere(2.0, 2.0, 1.0, 1.0, 0.0, 0.0));
  }

  #[test]
  fn it_calc_detect_collide_sphere_to_plane() {
    assert_eq!(true, detect_collide_sphere_to_plane(1.0, 1.0, 0.0));
    assert_eq!(false, detect_collide_sphere_to_plane(2.0, 1.0, 1.0));
  }
}