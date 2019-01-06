mod entity;
mod util;

/* We can define physics constants here */
static GRAVITY: f32 = 9.8;

const dt: f64 = 0.0;

fn main() {
    let mut t = 0.0;
    let mut current_time = std::time::Instant::now();
    let mut accumulator = 0.0;

    //Initialize our planes and spheres
    let state = entity::EntityState::new(1.0, 2.0, 3.0);
    let plane = entity::plane::Plane::new(state.clone(), 1.0, 2.0, 4.0);
    let mut all_entities: Vec<Box<entity::BaseEntity>> = Vec::new();
    all_entities.push(Box::new(plane));

    //Initialize world states
    let mut prev = entity::worldstate::WorldState::new(Vec::new());
    let mut curr = entity::worldstate::WorldState::new(all_entities);

    loop {
        let new_time = std::time::Instant::now();
        let mut frame_time = duration_to_s(new_time.duration_since(current_time)); // from ns to s
        if frame_time > 25 {
            // where did this constant come from?
            frame_time = 25;
        }
        current_time = new_time;

        accumulator += frame_time as f64;

        while accumulator >= dt {
            prev = curr.clone();
            curr.update_entities(t, dt);
            accumulator -= dt;
            t += dt;
        }

        let alpha = accumulator / dt;
        /* If we want to do operator overloading:
        let lerp_state: WorldState = curr * alpha + prev * (1.0 - alpha);
        lerp_state.print_state();
        */
    }
}

fn duration_to_s(duration: std::time::Duration) -> u64 {
    let nanos = duration.subsec_nanos() as u64;
    let s = (1000 * 1000 * 1000 * duration.as_secs() + nanos) / (1000 * 1000 * 1000);
    s
}
