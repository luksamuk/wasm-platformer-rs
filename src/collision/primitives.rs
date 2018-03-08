//! Submodule related to defining bounding volume primitives.

use types::Vector2;


/// Common trait for all objects which can interact
/// on a collision check.
pub trait Collidable<T> {
    /// Checks whether the object implementing this trait
    /// collides with another one.
    /// # Arguments
    /// * `other` - Reference to the other object, which impls
    /// this trait as well.
    fn collides(&self, other: &T) -> bool;

    
}

/// Common trait for all objects that can be contained
/// inside another primitive.
pub trait Delimitable {
    /// Yields a bounding circle for the collidable, which
    /// will prove useful in spatial partitioning situations.
    fn bounding_circle(&self) -> Circle;
}


// ================================

/// Represents an axis-aligned bounding box.
/// Stores its center point and its two half-sizes.
#[derive(Debug, Clone)]
pub struct AABB {
    pub center: Vector2,
    pub halfws: [f64; 2],
}

impl AABB {
    /// Yields the minimum point for the box
    /// (the top, leftmost one).
    pub fn min(&self) -> Vector2 {
        Vector2 {
            x: self.center.x - self.halfws[0],
            y: self.center.y - self.halfws[1]
        }
    }

    /// Yields the maximum point for the box
    /// (the bottom, rightmost one).
    pub fn max(&self) -> Vector2 {
        Vector2 {
            x: self.center.x + self.halfws[0],
            y: self.center.y + self.halfws[1]
        }
    }
}


// Collision implementations for AABB, interacting with other
// bounding volumes

impl Delimitable for AABB {
    fn bounding_circle(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.halfws[0].max(self.halfws[1]),
        }
    }
}

impl Collidable<AABB> for AABB {
    fn collides(&self, other: &AABB) -> bool {
        let mut r;
        
        r = self.halfws[0] + other.halfws[0];
        if (self.center.x - other.center.x + r) as u32 > (r + r) as u32 { return false; }

        r = self.halfws[1] + other.halfws[1];
        if (self.center.y - other.center.y + r) as u32 > (r + r) as u32 { return false; }
        
        true
    }
}

impl Collidable<Circle> for AABB {
    fn collides(&self, other: &Circle) -> bool {
        other.collides(self) // Already implemented
    }
}


// ================================

/// Represents a circle.
/// Stores its center point and its radius.
#[derive(Debug, Clone)]
pub struct Circle {
    pub center: Vector2,
    pub radius: f64,
}

// Collision implementations for Circle, interacting with other
// bounding volumes

impl Delimitable for Circle {
    fn bounding_circle(&self) -> Circle {
        self.clone()
    }
}

impl Collidable<Circle> for Circle {
    fn collides(&self, other: &Circle) -> bool {
        let d = self.center - other.center;
        let sq_dist = d.dot(d);

        let radius_sum = self.radius + other.radius;
        sq_dist <= (radius_sum * radius_sum)
    }
}

impl Collidable<AABB> for Circle {
    fn collides(&self, other: &AABB) -> bool {
        // Squared distance between circle and AABB
        let sq_dist = sqdist_vector2_aabb(&self.center, other);
        sq_dist <= self.radius * self.radius
    }
}




// ================================

// Extra primitive tests
// TODO: Give those operations their own module?

/// Returns the square distance between a given point
/// and the nearest point on an AABB.
/// # Arguments
/// * `p` - Reference to point
///
/// * `b` - Reference to AABB
fn sqdist_vector2_aabb(p: &Vector2, b: &AABB) -> f64 {
    let mut sq_dist = 0.0;
    let min = b.min();
    let max = b.max();
    
    // For X axis
    if p.x < min.x {
        sq_dist += (min.x - p.x) * (min.x - p.x);
    }
    if p.x > max.x {
        sq_dist += (p.x - max.x) * (p.x - max.x);
    }

    // For Y axis
    if p.y < min.y {
        sq_dist += (min.y - p.y) * (min.y - p.y);
    }
    if p.y > max.y {
        sq_dist += (p.y - max.y) * (p.y - max.y);
    }

    sq_dist
}
