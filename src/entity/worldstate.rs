use entity::BaseEntity;
use entity::Entity;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::Mul;
use std::time::Instant;

#[derive(Clone)]
pub struct WorldState {
    time: Instant,
    accumulator: f32,
    entities: HashMap<String, Entity>,
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
            entities: HashMap::new(),
            time: Instant::now(),
            accumulator: 0.0,
        }
    }

    pub fn new_with_map(entities: HashMap<String, Entity>) -> WorldState {
        WorldState {
            entities: entities,
            time: Instant::now(),
            accumulator: 0.0,
        }
    }

    pub fn add(&mut self, key: &str, ent: Entity) {
        self.entities.insert(String::from(key), ent);
    }

    pub fn update_entities(&mut self, dt: f32) {
        let mut new_entities: HashMap<String, Entity> = HashMap::new();
        for (key, ent) in &self.entities {
            new_entities.insert(key.clone(), (*ent).update_state(self.accumulator, dt));
        }

        let entities_copy = self.entities.clone();
        for (key, ent) in &mut self.entities {
            for (key2, ent2) in &entities_copy {
                if *key == *key2 {
                    continue;
                }
                if (ent.is_collided(ent2.clone())) {
                    ent.collide_with_entity(ent2.clone());
                }
            }
        }
        self.entities = new_entities;
    }

    pub fn print_state(&self) {
        for (_, ent) in &self.entities {
            ent.print();
        }
    }

    pub fn get(&self, key: String) -> Entity {
        self.entities.get(&key).expect("Should exist").clone()
    }

    pub fn step(&mut self, dt: f32) {
        let mut prev = self.clone();
        let new_time = Instant::now();
        let frame_time = 1;
        // let mut frame_time = duration_to_s(new_time.duration_since(self.time)); // from ns to s
        // if frame_time > 25 {
        //     // where did this constant come from?
        //     frame_time = 25;
        // }
        self.time = new_time;

        self.accumulator += frame_time as f32;

        while self.accumulator >= dt {
            prev = self.clone();
            self.update_entities(dt);
            self.accumulator -= dt;
        }

        // Need to implement add overload for world state
        let alpha = self.accumulator / dt;
        let lerp_state: WorldState = self.clone() * alpha + prev.clone() * (1.0 - alpha);
        lerp_state.print_state();
    }
}

impl Mul<f32> for WorldState {
    type Output = WorldState;

    fn mul(self, _rhs: f32) -> WorldState {
        let mut lerp_ents = HashMap::<String, Entity>::new();
        for (key, ent) in self.entities {
            let new_ent = ent.new_entity_with_state(ent.get_next_position(_rhs));
            lerp_ents.insert(key, new_ent);
        }
        WorldState::new_with_map(lerp_ents)
    }
}

impl Add for WorldState {
    type Output = WorldState;

    fn add(self, other: WorldState) -> WorldState {
        let mut lerp_ents = HashMap::<String, Entity>::new();
        for (key, ent) in self.entities {
            let other_ent = other.entities.get(&key);
            if other_ent.is_none() {
                lerp_ents.insert(key, ent);
            } else {
                let new_pos = *(ent).get_position() + *(other_ent.unwrap()).get_position();
                let mut new_ent = ent.clone();
                new_ent.set_position(new_pos);
                lerp_ents.insert(key, new_ent);
            }
        }
        WorldState::new_with_map(lerp_ents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::sphere::Sphere;
    use util::vector3::Vector3;

    #[test]
    fn it_update_entities() {
        let init_pos_1 = Vector3::new(2.0, 2.0, 2.0);
        let v1 = Vector3::new(1.0, 1.0, 1.0);
        let m = 1.0;
        let r = 1.0;
        let sphere1 = Sphere::new(String::from("Sphere1"), init_pos_1, m, r, v1);
        let mut all_entities: HashMap<String, Entity> = HashMap::new();
        all_entities.insert(String::from("Sphere1"), Entity::Sphere(sphere1));
        let mut state = WorldState::new_with_map(all_entities);

        let dt = 1.0;
        state.update_entities(dt);

        let sphere1_box = state.get(String::from("Sphere1"));

        let expected = Vector3::new(3.0, -16.6, 3.0);
        assert_eq!(&expected, sphere1_box.get_position());
    }
}
