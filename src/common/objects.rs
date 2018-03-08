//! Submodule for common game objects.

use types::Vector2;
use collision::primitives::Circle;
use render::Renderer2D;

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a dynamic reference to a `GameObject`.
pub type GameObjectRef = Rc<RefCell<GameObject>>;

/// Represents a dynamic reference to any object.
pub type ObjectRef<T> = Rc<RefCell<T>>;

/// Common trait to a game object component.
pub trait GameComponent {
}

/// Common trait to a game object.
pub trait GameObject {
    /// Performs destructive routines on this object's
    /// assets when it is disposed, if necessary.
    fn unload(&mut self) { }

    /// Performs logic update routines for this
    /// game object.
    /// # Arguments
    /// * `dt` - Time difference between the last frame
    /// and the current frame, in seconds. The value is
    /// likely lower than 1.0.
    fn update(&mut self, dt: f64);

    /// Performs on-screen drawing routines for
    /// this game object.
    /// # Arguments
    /// * `_renderer` - Reference to the renderer used.
    fn draw(&mut self, _renderer: &Renderer2D) { }

    /// Yields the bounding circle of this object,
    /// used on partitioning and collision operations.
    ///
    /// TODO: This might be replaced by the `Delimitable`
    /// trait.
    fn bounding_circle(&self) -> Circle;

    /// Yields the current position of this object.
    fn get_position(&self) -> Vector2;
    
    /// Yields a number identifying this object's instance.
    fn get_id(&self) -> u32;

    /// Collision callback for whenever two objects' bounding circles
    /// overlap. This callback is called once per
    /// object on collision event, and collisions are never
    /// repeated (e.g. a collision between A and B is the same as
    /// a collision between B and A).
    /// # Arguments
    /// * `other`- Dynamic reference to the other game object.
    fn on_collision(&mut self, other: ObjectRef<GameObject>);
}

/// Wraps an object into a dynamic reference.
/// # Arguments
/// * `object` - Object to be dynamically wrapped.
pub fn wrap_to_ref<T>(object: T) -> ObjectRef<T> {
    Rc::new(RefCell::new(object))
}
