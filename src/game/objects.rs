use types::Vector2;
use collision::primitives::Circle;
use common::objects::{ GameObject, GameObjectRef };
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

    counter:  f64,
    collided: (u32, u32),
}

impl Entity {
    pub fn new(id: u32, position: Vector2, color: &str) -> Entity {
        Entity {
            id: id,
            position: position,
            color:    if id == 0 { String::from("#112233") } else { String::from(color) },
            radius:   50.0,
            counter:  0.0,
            collided: (0, 0),
        }
    }
}

impl GameObject for Entity {
    fn unload(&mut self) {
    }
    
    fn update(&mut self, dt: f64) {
        //println!("Object #{} performing logic update", self.id);
        if self.id == 0 {
            self.counter = ((self.counter as u32 + 1) % 360) as f64;
            self.position.x = 250.0 + (130.0 * self.counter.to_radians().cos());
            self.position.y = 250.0 + (130.0 * self.counter.to_radians().sin());

            if self.collided.0 != self.collided.1 {
                println!("Obj #0 colliding w/ {} objects", self.collided.0);
            }
            self.collided.1 = self.collided.0;
            self.collided.0 = 0;
        }
    }

    fn draw(&mut self, renderer: &Renderer2D) {
        renderer.draw_circle(self.color.as_ref(), self.position, self.radius);
    }

    fn bounding_circle(&self) -> Circle {
        Circle { center: self.position, radius: self.radius }
    }

    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn on_collision(&mut self, other: GameObjectRef) {
        if self.id == 0 {
            //println!("Collided #{} with other entity", self.id);
            self.collided.0 += 1;
        }
    }
}
