use entity;
use std::ops::{Mul};

#[derive(Debug)]
pub struct WorldState {
    entities: Vec<&entity::BaseEntity>,
}

impl WorldState {
    pub fn new(entities: Vec<&entity::BaseEntity>) -> WorldState {
        return WorldState {
            entities: entities
        };
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
        let mut lerp_ents = Vec::<&entity::BaseEntity>::new(); 
        for ent in self.entities {
            lerp_ents.push(ent.get_next_state(_rhs));
        }
        lerp_ents
    }
}
