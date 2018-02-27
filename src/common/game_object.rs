use collision::primitives::Circle;
use game::objects::*;
use render::Renderer2D;

use std::rc::Rc;
use std::cell::RefCell;

pub type GameObjectRef = Rc<RefCell<GameObject>>;

pub type ObjectRef<T> = Rc<RefCell<T>>;

pub trait GameComponent {
}

pub trait GameObject {
    fn unload(&mut self);
    fn update(&mut self, dt: f64);
    fn draw(&mut self, renderer: &Renderer2D);

    fn bounding_circle(&self) -> Circle;
    
    fn on_collision(&mut self, other: ObjectRef<GameObject>);
}


pub fn wrap_to_ref<T>(object: T) -> ObjectRef<T> {
    Rc::new(RefCell::new(object))
}
