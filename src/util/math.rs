use super::vector3::Vector3;
use entity::sphere::Sphere;
use entity::BaseEntity;

const COEFFICIENT_OF_FRICTION: f32 = 0.05;
const ACC_GRAVITY: f32 = 9.8;
const COEFFICITION_OF_RESTITUTION_SPHERE: f32 = 1.0;

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

pub fn detect_collide_sphere_to_plane(center: Vector3, radius: f32, bmin: Vector3, bmax: Vector3) -> bool {
    //https://stackoverflow.com/questions/15247347/collision-detection-between-a-boundingbox-and-a-sphere-in-libgdx
    let mut dmin = 0.0;

    if (center.x < bmin.x) {
        dmin += (center.x - bmin.x).powf(2.0);
    } else if (center.x > bmax.x) {
        dmin += (center.x - bmax.x).powf(2.0);
    }

    if (center.y < bmin.y) {
        dmin += (center.y - bmin.y).powf(2.0);
    } else if (center.y > bmax.y) {
        dmin += (center.y - bmax.y).powf(2.0);
    }

    if (center.z < bmin.z) {
        dmin += (center.z - bmin.z).powf(2.0);
    } else if (center.z > bmax.z) {
        dmin += (center.z - bmax.z).powf(2.0);
    }

    return dmin <= (radius).powf(2.0);
}

pub fn calculate_impulse_force(sphere1: &Sphere, sphere2: &Sphere) -> Vector3 {
    //finally found formula at
    //https://www.gamasutra.com/view/feature/3168/physics_on_the_back_of_a_cocktail_.php?print=1
    let relative_velocity = sphere1.get_velocity() - sphere2.get_velocity();
    let dir_of_impact = Vector3::new(sphere2.get_position().x - sphere1.get_position().x, sphere2.get_position().y - sphere1.get_position().y, sphere1.get_position().z - sphere2.get_position().z).normalized();
    let numerator = (relative_velocity * -(1.0 + COEFFICITION_OF_RESTITUTION_SPHERE)).dot_product(dir_of_impact);
    return dir_of_impact * (numerator * (1.0 / ((1.0 / sphere1.get_mass()) + (1.0 / sphere2.get_mass ()))));
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

  /* tested elsewhere now
  #[test]
  fn it_calc_detect_collide_sphere_to_plane() {
    assert_eq!(true, detect_collide_sphere_to_plane(1.0, 1.0, 0.0));
    assert_eq!(false, detect_collide_sphere_to_plane(2.0, 1.0, 1.0));
  }
  */
}

