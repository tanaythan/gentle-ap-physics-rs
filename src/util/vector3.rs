use std::ops;
use std::cmp;

#[derive(Debug, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        return Vector3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z);
    }
}


impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        return Vector3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z);
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {
        return Vector3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs);
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Vector3 {
        return Vector3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs);
    }
}

impl cmp::PartialEq<Vector3> for Vector3 {

    fn eq(&self, other: &Vector3) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        return Vector3 { x: x, y: y, z: z };
    }

    pub fn magnitude (&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    pub fn normalized (&self) -> Vector3 {
        let mag = self.magnitude();
        return Vector3::new (self.x / mag, self.y / mag, self.z / mag);
    }

    pub fn dot_product (&self, other: Vector3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}
