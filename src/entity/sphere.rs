use entity;
use entity::plane::Plane;
use entity::three::Object;
use entity::BaseEntity;
use entity::Entity;
use std::collections::HashSet;
use std::collections::HashMap;
use util::math;
use util::vector3::Vector3;

fn get_g_force(mass: f32) -> Vector3 {
    Vector3::new(0.0, -1.0 * math::gravity(mass), 0.0)
}

#[derive(Debug, Clone)]
pub struct Sphere {
    name: String,
    position: Vector3,
    mass: f32,
    radius: f32,
    velocity: Vector3,
    forces: Vec<Vector3>,
    mesh: Option<three::Mesh>,
    collision: HashSet<String>,
}

impl Sphere {
    pub fn new(
        name: String,
        position: Vector3,
        mass: f32,
        radius: f32,
        velocity: Vector3,
    ) -> Sphere {
        Sphere {
            name: name,
            position: position,
            mass: mass,
            radius: radius,
            velocity: velocity,
            forces: [get_g_force(mass)].to_vec(),
            mesh: None,
            collision: HashSet::new(),
        }
    }

    pub fn set_graphics(&mut self, window: &mut three::Window) {
        let mesh = {
            let geometry = three::Geometry::uv_sphere(self.radius, 1000, 1000);
            let material = three::material::Wireframe { color: 0xFF0000 };
            window.factory.mesh(geometry, material)
        };
        window.scene.add(&mesh);
        self.mesh = Some(mesh);
    }

    pub fn is_collided(&mut self, other: &Entity) -> bool {
        let res = match other {
            Entity::Sphere(sphere) => math::detect_collide_sphere_to_sphere(
                self.position,
                sphere.position,
                self.radius,
                sphere.radius,
            ),
            Entity::Plane(plane) => math::detect_collide_sphere_to_plane(
                self.position,
                self.radius,
                plane.get_min_point(),
                plane.get_max_point(),
            ),
        };
        if self.collision.contains(&other.get_name()) {
            if res {
                return false;
            } else {
                self.collision.remove(&other.get_name());
            }
        } else if res {
            self.collision.insert(other.get_name());
        }
        res
    }

    pub fn is_colliding(&self, other: &Entity) -> bool {
        self.collision.contains(&other.get_name())
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn collide_with_sphere(&mut self, other: &mut Sphere) {
        let force = math::calculate_impulse_force_between_spheres(&self, &other);
        self.apply_force(force);

        //other is a clone, no need to apply
        //other.apply_force(force * -1.0);
    }

    pub fn collide_with_plane(&mut self, other: Plane) {
        let force = math::calculate_impulse_force_sphere_plane(&self, &other);
        self.apply_force(force);
    }

    pub fn get_velocity(&self) -> Vector3 {
        self.velocity
    }

    pub fn render(&self) {
        match self.mesh {
            Some(ref mesh) => {
                mesh.set_position([self.position.x, self.position.y, self.position.z])
            }
            None => panic!("Can't render w/o graphics!"),
        }
    }
}

impl entity::BaseEntity for Sphere {
    fn set_position(&mut self, position: Vector3) {
        self.position = position;
    }

    fn get_position(&self) -> &Vector3 {
        &self.position
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_next_position(&self, dt: f32) -> Vector3 {
        math::new_pos(self.position, self.get_next_velocity(dt), dt)
    }

    fn new_entity_with_state(&self, _entity: Vector3) -> Entity {
        Entity::Sphere(self.clone())
    }

    fn update_state(&self, _t: f32, dt: f32) -> Entity {
        let mut sphere = self.clone();
        sphere.velocity = sphere.get_next_velocity(dt);
        sphere.position = sphere.get_next_position(dt);
        sphere.forces = [get_g_force(sphere.mass)].to_vec();
        Entity::Sphere(sphere)
    }

    fn print(&self) {
        println!("SPHERE: ");
        println!("\t Name: {:?}", self.name);
        println!("\t Position: {:?}", self.position);
        println!("\t Mass: {:?}", self.mass);
        println!("\t Radius: {:?}", self.radius);
        println!("\t Velocity: {:?}", self.velocity);
        let mut f_map : HashMap<Vector3, i32> =  HashMap::new();
        for f in &self.forces {
            *f_map.entry(*f).or_insert(0) += 1;
        }
        println!("\t Forces: {:?}", f_map);
    }

    fn get_net_acceleration(&self) -> Vector3 {
        let mut a = Vector3::new(0.0, 0.0, 0.0);
        for f in &self.forces {
            a = a + *f;
        }
        a
    }

    fn get_next_velocity(&self, dt: f32) -> Vector3 {
        let net_accel = self.get_net_acceleration();
        math::velocity_from_acc(self.velocity, net_accel, dt)
    }

    fn apply_force(&mut self, f: Vector3) {
        self.forces.push(f)
    }

    fn collide_with_entity(&mut self, other: Entity) {
        match other {
            Entity::Sphere(mut sphere) => self.collide_with_sphere(&mut sphere),
            Entity::Plane(plane) => self.collide_with_plane(plane),
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::sphere::Sphere;
    use entity::BaseEntity;

    #[test]
    fn it_is_collided() {
        let vec = Vector3::new(1.0, 1.0, 1.0);
        let mut sphere1 = Sphere::new("Sphere1".to_string(), vec, 1.0, 1.0, vec);
        let sphere2 = Sphere::new("Sphere2".to_string(), vec, 1.0, 1.0, vec);
        assert_eq!(true, sphere1.is_collided(&Entity::Sphere(sphere2)));
    }

    #[test]
    fn it_plane_collisions() {
        let vec = Vector3::new(1.0, 1.0, 1.0);
        let mut sphere1 = Sphere::new("Sphere1".to_string(), vec, 1.0, 1.0, vec);
        let plane1 = entity::plane::Plane::new("Plane1".to_string(), vec, 1.0, 1.0, 1.0);
        assert_eq!(true, sphere1.is_collided(&Entity::Plane(plane1)));
        let plane2 = entity::plane::Plane::new(
            "Plane2".to_string(),
            Vector3::new(4.0, 4.0, 4.0),
            1.0,
            1.0,
            1.0,
        );
        assert_eq!(false, sphere1.is_collided(&Entity::Plane(plane2)));
    }

    #[test]
    fn it_apply_force() {
        let m = 1.0;
        let vec = Vector3::new(1.0, 1.0, 1.0);
        let mut sphere1 = Sphere::new("Sphere1".to_string(), vec, m, 1.0, vec);
        let f = Vector3::new(1.0, 0.0, 1.0);
        sphere1.apply_force(f);
        let expected = f + get_g_force(m);
        assert_eq!(expected, sphere1.get_net_acceleration());
    }
}
