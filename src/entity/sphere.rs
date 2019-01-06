use entity;

const ACC_GRAVITY: f32 = 9.8;


#[derive(Debug)]
pub struct Sphere {
    position: entity::Vector3,
    mass: f32,
    radius: f32
    velocity: entity:Vector3,
}

impl Sphere {
    pub fn new(position: entity::Vector3, mass: f32, radius: f32, velocity: entity::Vector3) -> Sphere {
        return Sphere {
            position: position,
            mass: mass,
            radius: radius,
            velocity: velocity,
        };
    }
}

impl Clone for Sphere {
    fn clone(&self) -> Sphere {
        Sphere {
            position: self.position.clone (),
            mass: self.mass,
            radius: self.radius,
            velocity: self.velocity.clone (),
        }
    }
}

impl entity:BaseEntity for Sphere {
  fn set_position(&mut self, position: entity::Vector3) {
    self.position = position.clone();
  }

  fn get_position(&self) -> &entity::Vector3 {
    &self.position
  }

  fn get_mass(&self) -> f32 {
    return self.mass;
  }

  fn get_next_position(&self, dt: f32) -> entity::Vector3 {
    return self.position + self.get_next_velocity * dt;
  }

  fn new_entity_with_state(&self, entity: entity::Vector3) -> Box<entity::BaseEntity> {
    let sphere = self.clone();
    Box::new(sphere)
  }

  fn update_state(&self, t: f32, dt: f32) {
    self.velocity = self.get_next_velocity;
    self.position = self.get_next_position;
  }

  fn print(&self) {}

  fn get_net_acceleration(&self) -> Vector3{
    return entity::Vector3::new (0.0, self.mass * ACC_GRAVITY, 0.0);
  }

  fn get_next_velocity(&self, dt: f32) -> entity::Vector3 {
    let net_accel = self.get_net_acceleration ();
    return self.velocity + net_accel * dt;
  }

}
