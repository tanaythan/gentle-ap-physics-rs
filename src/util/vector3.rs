use std::cmp;
use std::ops;
use std::arch::x86_64::*;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        unsafe {
            let x = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let y = _mm_setr_ps(_rhs.x, _rhs.y, _rhs.z, 0.0);
            let z = _mm_add_ps(x, y);
            let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let array_ptr: *mut f32 = &mut result[0] as *mut f32;
            _mm_storeu_ps(array_ptr, z);
            return Vector3::new(result [0], result[1], result[2]);
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        unsafe {
            let x = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let y = _mm_setr_ps(-_rhs.x, -_rhs.y, -_rhs.z, 0.0);
            let z = _mm_add_ps(x, y);
            let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let array_ptr: *mut f32 = &mut result[0] as *mut f32;
            _mm_storeu_ps(array_ptr, z);
            return Vector3::new(result [0], result[1], result[2]);
        }    
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {
        unsafe {
            let x = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let y = _mm_setr_ps(_rhs, _rhs, _rhs, 0.0);
            let z = _mm_mul_ps(x, y);
            let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let array_ptr: *mut f32 = &mut result[0] as *mut f32;
            _mm_storeu_ps(array_ptr, z);
            return Vector3::new(result [0], result[1], result[2]);
        }    
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Vector3 {
        unsafe {
            let x = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let y = _mm_setr_ps(1.0 / _rhs, 1.0 / _rhs, 1.0 / _rhs, 0.0);
            let z = _mm_mul_ps(x, y);
            let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let array_ptr: *mut f32 = &mut result[0] as *mut f32;
            _mm_storeu_ps(array_ptr, z);
            return Vector3::new(result [0], result[1], result[2]);
        }    
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

    pub fn magnitude(&self) -> f32 {
        unsafe {
            let x = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let y = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let z = _mm_mul_ps(x, y);
            let sum1 = _mm_hadd_ps(z, z);
            let sum2 = _mm_hadd_ps(sum1, sum1);
            let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let array_ptr: *mut f32 = &mut result[0] as *mut f32;
            _mm_storeu_ps(array_ptr, sum2);
            return result[0].sqrt();
        }    
    }

    pub fn normalized(&self) -> Vector3 {
        let mag = self.magnitude();
        unsafe {
            let x = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let y = _mm_setr_ps(1.0 / mag, 1.0 / mag, 1.0 / mag, 0.0);
            let z = _mm_mul_ps(x, y);
            let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let array_ptr: *mut f32 = &mut result[0] as *mut f32;
            _mm_storeu_ps(array_ptr, z);
            return Vector3::new(result [0], result[1], result[2]);
        }
    }

    pub fn dot_product(&self, other: Vector3) -> f32 {
        unsafe {
            let x = _mm_setr_ps(self.x, self.y, self.z, 0.0);
            let y = _mm_setr_ps(other.x, other.y, other.z, 0.0);
            let z = _mm_mul_ps(x, y);
            let sum1 = _mm_hadd_ps(z, z);
            let sum2 = _mm_hadd_ps(sum1, sum1);
            let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let array_ptr: *mut f32 = &mut result[0] as *mut f32;
            _mm_storeu_ps(array_ptr, sum2);
            return result[0];
        }    
    }
}
