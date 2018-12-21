use entity;
use std::ops::Mul;

pub struct WorldState {
    entities: Vec<Box<entity::BaseEntity>>,
}

impl WorldState {
    pub fn new(entities: Vec<Box<entity::BaseEntity>>) -> WorldState {
        return WorldState { entities: entities };
    }

    pub fn update_entities(&self, time: f32, dt: f32) {
        for ent in &self.entities {
            ent.update_state(time, dt);
        }
    }

    pub fn print_state(&self) {
        for ent in &self.entities {
            ent.print();
        }
    }
}

impl Clone for WorldState {
    fn clone(&self) -> WorldState {
        let mut new_entities: Vec<Box<entity::BaseEntity>> = Vec::new();
        for entity in &self.entities {
            let new_ent = (*entity).clone();
            new_entities.push(new_ent);
        }
        WorldState {
            entities: new_entities,
        }
    }
}

impl Mul<f32> for WorldState {
    type Output = WorldState;

    fn mul(self, _rhs: f32) -> WorldState {
        let mut lerp_ents = Vec::<Box<entity::BaseEntity>>::new();
        for ent in self.entities {
            let new_ent = ent.new_entity_with_state(ent.get_next_position(_rhs));
            lerp_ents.push(new_ent);
        }
        WorldState::new(lerp_ents)
    }
}
