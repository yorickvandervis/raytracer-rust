use std::fmt;
use std::fmt::Display;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

// Constructor
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
}

// Overwrite operators
impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

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

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) -> () {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        };
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) -> () {
        *self = Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other],
        };
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

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other[0], self * other[1], self * other[2]],
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

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) -> () {
        *self = Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        };
    }
}

// utility functions for Vec3
impl Vec3 {
    pub fn x(self) -> f32 {
        self[0]
    }

    pub fn y(self) -> f32 {
        self[1]
    }

    pub fn z(self) -> f32 {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn format_color(self) -> String {
        format!("{} {} {}", (255.999 * self[0]) as u32,
                            (255.999 * self[1]) as u32,
                            (255.999 * self[2]) as u32)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}



// Basic test setup
#[cfg(test)]
mod tests {
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
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) * 2.0, Vec3::new(4.0, 8.0, 12.0))
    }

    #[test]
    fn test_vec3_division() {}

    #[test]
    fn test_vec3_add_assign() {}

    #[test]
    fn test_vec3_mul() {}

    #[test]
    fn test_f32_mul() {}

    #[test]
    fn test_vec3_mull_assign() {}

    #[test]
    fn test_vec3_div() {}

    #[test]
    fn test_vec3_div_assign() {}
}
