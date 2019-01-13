use entity;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::Mul;

pub struct WorldState {
    time: f32,
    entities: HashMap<String, Box<entity::BaseEntity>>,
}

impl WorldState {
    pub fn new(entities: HashMap<String, Box<entity::BaseEntity>>) -> WorldState {
        return WorldState { entities: entities, time: 0.0 };
    }

    pub fn update_entities(&mut self, dt: f32) -> WorldState {
        let mut new_entities: HashMap<String, Box<entity::BaseEntity>> = HashMap::new();
        for (key, ent) in &self.entities {
            new_entities.insert(key.clone(), (*ent).update_state(self.time, dt));
        }
        let mut new_state = self.clone();
        new_state.entities = new_entities;
        self.time += dt;
        new_state
    }

    pub fn print_state(&self) {
        for (_, ent) in &self.entities {
            ent.print();
        }
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
        WorldState::new(lerp_ents)
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
        WorldState::new(lerp_ents)
    }
}
