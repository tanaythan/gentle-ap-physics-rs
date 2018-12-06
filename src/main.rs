mod entity;

fn main() {
    let state = entity::EntityState::new(1.0, 2.0, 3.0);
    let plane = entity::plane::Plane::new(state.clone(), 1.0, 2.0, 4.0);
    println!("{:?}", state);
    println!("{:?}", plane);
}
