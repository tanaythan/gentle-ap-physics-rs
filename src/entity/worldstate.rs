use entity;
use entity::BaseEntity;
use std::ops::Mul;

pub struct WorldState {
    entities: Vec<Box<entity::BaseEntity>>,
}

impl WorldState {
    pub fn new(entities: Vec<Box<entity::BaseEntity>>) -> WorldState {
        return WorldState { entities: entities };
    }

    pub fn update_entities(&self, time: f64, dt: f64) {
        for ent in self.entities {
            ent.update_state(time, dt);
        }
    }

    pub fn print_state(&self) {
        for ent in self.entities {
            ent.print();
        }
    }
}

impl Mul<f64> for WorldState {
    type Output = WorldState;

    fn mul(self, _rhs: f64) -> WorldState {
        let mut lerp_ents = Vec::<Box<entity::BaseEntity>>::new();
        for ent in self.entities {
            let new_ent = ent.new_entity_with_state(ent.get_next_state(_rhs));
            lerp_ents.push(new_ent);
        }
        WorldState::new(lerp_ents)
    }
}
