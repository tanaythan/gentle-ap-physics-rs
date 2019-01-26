use entity;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::Mul;
use util::vector3::Vector3;
use util::time::duration_to_s;
use std::time::Instant;

pub struct WorldState {
    time: Instant,
    accumulator: f32,
    entities: HashMap<String, Box<entity::BaseEntity>>,
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
            entities: HashMap::new(),
            time: Instant::now(),
            accumulator: 0.0
        }
    }

    pub fn new_with_map(entities: HashMap<String, Box<entity::BaseEntity>>) -> WorldState {
        WorldState {
            entities: entities,
            time: Instant::now(),
            accumulator: 0.0
        }
    }
    pub fn add(&mut self, key: &str, ent: Box<entity::BaseEntity>) {
        self.entities.insert(String::from(key), ent);
    }

    pub fn update_entities(&mut self, dt: f32) {
        let mut new_entities: HashMap<String, Box<entity::BaseEntity>> = HashMap::new();
        for (key, ent) in &self.entities {
            new_entities.insert(key.clone(), (*ent).update_state(self.accumulator, dt));
        }
    }

    pub fn print_state(&self) {
        for (_, ent) in &self.entities {
            ent.print();
        }
    }

    pub fn get(&self, key: String) -> Box<entity::BaseEntity> {
        self.entities.get(&key).unwrap().clone()
    }

    pub fn step(&mut self, dt: f32) {
        let mut prev = self.clone();
        let new_time = std::time::Instant::now();
        let mut frame_time = duration_to_s(new_time.duration_since(self.time)); // from ns to s
        if frame_time > 25 {
            // where did this constant come from?
            frame_time = 25;
        }
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

impl Clone for WorldState {
    fn clone(&self) -> WorldState {
        let mut new_entities: HashMap<String, Box<entity::BaseEntity>> = HashMap::new();
        for (key, entity) in &self.entities {
            let new_ent = entity.clone();
            new_entities.insert(key.clone(), new_ent);
        }
        WorldState {
            time: self.time,
            entities: new_entities,
            accumulator: self.accumulator
        }
    }
}

impl Mul<f32> for WorldState {
    type Output = WorldState;

    fn mul(self, _rhs: f32) -> WorldState {
        let mut lerp_ents = HashMap::<String, Box<entity::BaseEntity>>::new();
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
        let mut lerp_ents = HashMap::<String, Box<entity::BaseEntity>>::new();
        for (key, ent) in self.entities {
            let other_ent = other.entities.get(&key);
            if other_ent.is_none() {
                lerp_ents.insert(key, ent);
            } else {
                let new_pos = *(*ent).get_position() + *(*other_ent.unwrap()).get_position();
                let mut new_ent = ent.clone();
                new_ent.set_position(new_pos);
                lerp_ents.insert(key, new_ent);
            }
        }
        WorldState::new_with_map(lerp_ents)
    }
}
