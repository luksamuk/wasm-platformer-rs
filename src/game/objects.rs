use types::Vector2;
use collision::primitives::Collidable;
use collision::primitives::Circle;
use common::game_object::{ GameObject, GameObjectRef };
use std::cell::RefCell;
use stdweb::web::CanvasRenderingContext2d;
use render::Renderer2D;

#[derive(Debug)]
pub enum EntityType {
    Player,
}

pub struct Entity {
    id:       u32,
    position: Vector2,
    color:    String,
    radius:   f64,
}

impl Entity {
    pub fn new(id: u32, position: Vector2, color: &str) -> Entity {
        Entity {
            id: id,
            position: position,
            color:    String::from(color),
            radius:   50.0
        }
    }
}

impl GameObject for Entity {
    fn unload(&mut self) {
    }
    
    fn update(&mut self, dt: f64) {
        println!("Object #{} performing logic update", self.id);
    }

    fn draw(&mut self, renderer: &Renderer2D) {
        renderer.draw_circle(self.color.as_ref(), self.position, self.radius);
    }

    fn bounding_circle(&self) -> Circle {
        Circle { center: self.position, radius: self.radius }
    }

    fn on_collision(&mut self, other: GameObjectRef) {
        println!("Collided #{} with other entity", self.id);
    }
}
