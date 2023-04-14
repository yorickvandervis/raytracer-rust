use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

// Constructor
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
}

// Overwrite addition operator to add Vec3 together
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, right_hand_side: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + right_hand_side.e[0],
                self.e[1] + right_hand_side.e[1],
                self.e[2] + right_hand_side.e[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, right_hand_side: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * right_hand_side, self.e[1] * right_hand_side, self.e[2] * right_hand_side],
        }
    }
}

// Basic test setup
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vec3_add() {}

    #[test]
    fn test_vec3_multiply() {}

    #[test]
    fn test_vec3_division() {}

}