mod entity;
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
    let all_entities = Vec::<entity::BaseEntity>::new();
    all_entities.push(&plane);

    //Initialize world states
    let mut prev = entity::worldstate::WorldState::new(all_entities);
    let mut curr = entity::worldstate::WorldState::new(all_entities);
    
    while true {
        let new_time = std::time::Instant::now();
        let mut frame_time = new_time.from(current_time).as_nanos / 1000000000; // from ns to s
        if frame_time > 0.25 { // where did this constant come from?
            frame_time = 0.25;
        }
        current_time = new_time;

        accumulator += frame_time;

        while accumulator >= dt {
            prev = curr;
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
