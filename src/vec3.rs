use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

// Constructor
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
}

// Overwrite addition/division/multiply operators
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let k = 1.0 / rhs;

        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        }
    }
}

// Basic test setup
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(3.5, 4.1, 5.7) + Vec3::new(1.5, 1.6, 5.2),
            Vec3::new(5.0, 5.7, 10.9)
        )
    }

    #[test]
    fn test_vec3_multiply() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) * 2.0,
            Vec3::new(4.0, 8.0, 12.0)
        )
    }

    #[test]
    fn test_vec3_division() {}

}