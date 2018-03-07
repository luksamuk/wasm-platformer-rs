//! Module providing common types for the entire engine.

use std::ops;
use std::cmp::PartialEq;


// Vector2: 2D vector
// =============================

/// Represents a 2D vector.
/// This struct is stored in memory the same way that
/// a C struct is, so that, if needed, we can cast this
/// to an array.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

// Constructors
impl Vector2 {
    /// Creates a new 2D vector.
    /// # Arguments
    /// `x` - X coordinate.
    /// `y` - Y coordinate.
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    
    /// Creates a new 2D vector pointing to the
    /// origin.
    pub fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
    
    /// Creates a new 2D vector with unitary
    /// axes.
    pub fn one() -> Vector2 {
        Vector2 { x: 1.0, y: 1.0 }
    }
}

// Vector2 arithmetic
// TODO: Shall we create a "Vector" trait?
impl Vector2 {
    /// Returns the dot product between this vector
    /// and another one. Returns a scalar with the
    /// dot product.
    /// # Arguments
    /// `rhs` - Second vector for performing the operation.
    pub fn dot(&self, rhs: Vector2) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}


// Additional Vector2D arithmetic (operator overloading)
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
