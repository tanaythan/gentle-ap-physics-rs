const COEFFICIENT_OF_FRICTION: f64 = 0.05;
const ACC_GRAVITY: f64 = 9.8;

pub fn friction(f: f64) -> f64 {
    COEFFICIENT_OF_FRICTION * f
}

pub fn gravity(m: f64) -> f64 {
    m * ACC_GRAVITY
}

pub fn velocity(src_pos: f64, dst_pos: f64, t: f64) -> f64 {
    (dst_pos - src_pos) / t
}

pub fn new_pos(src_pos: f64, v: f64, t: f64) -> f64 {
    v * t + src_pos
}

fn sqr(x: f64) -> f64 {
    x * x
}

pub fn detect_collide_sphere_to_sphere(x1: f64, y1: f64, x2: f64, y2: f64, 
                                       r1: f64, r2: f64) -> bool {
    sqr(x1 - x2) + sqr(y1 - y2) <= sqr(r1 + r2)
}

pub fn detect_collide_sphere_to_plane(center: f64, plane: f64, 
                                      normal: f64) -> bool {
    (center - plane) * normal <= 0.0
}
