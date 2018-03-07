use types::Vector2;
use collision::primitives::Circle;
use common::objects::{ GameObject, GameObjectRef };
use render::Renderer2D;
use stdweb::unstable::TryInto;

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
    change_color: bool,
    original:   Vector2,
    speed:    f64,
}

impl Entity {
    pub fn new(id: u32, position: Vector2, color: &str) -> Entity {
        Entity {
            id: id,
            position: position,
            color:    if id == 0 { String::from("#112233") } else { String::from(color) },
            radius:   if id == 0 { 20.0 } else { 50.0 },
            counter:  0.0,
            collided: (0, 0),
            change_color: false,
            original: position,
            speed:    js! { return 1.0 + (Math.random() * 10.0); }.try_into().unwrap(),
        }
    }
}

impl GameObject for Entity {
    fn unload(&mut self) {
    }
    
    fn update(&mut self, dt: f64) {
        if self.id == 0 {
            // Small circle moves rapidly
            self.counter = ((self.counter as u32 + 1) % 360) as f64;
            let distance = 250.0 * (self.counter * 2.0).to_radians().sin();
            self.position.x = 250.0 + (distance * self.counter.to_radians().cos());
            self.position.y = 250.0 + (distance * self.counter.to_radians().sin());
        } else {
            // Bigger circles move gracefully
            self.counter = ((self.counter as u32 + 1) % 1440) as f64;
            let distance = (self.counter / 4.0).to_radians();
            self.position.x = self.original.x + 20.0 * distance.sin()
                * if self.id % 2 == 0 { -1.0 } else { 1.0 } * self.speed;
            self.position.y = self.original.y + 20.0 * distance.cos()
                * if self.id % 2 == 0 { 1.0 } else { -1.0 } * self.speed;
            self.change_color = self.collided.0 != 0;
        }

        self.collided.1 = self.collided.0;
        self.collided.0 = 0;
    }

    fn draw(&mut self, renderer: &Renderer2D) {
        renderer.draw_circle(
            if self.id != 0 && self.change_color { "#1122337f" } else { self.color.as_ref() },
            self.position,
            self.radius);
    }

    fn bounding_circle(&self) -> Circle {
        Circle { center: self.position, radius: self.radius }
    }

    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn on_collision(&mut self, other: GameObjectRef) {
        if other.borrow().get_id() == 0 {
            self.collided.0 += 1;
        }
    }
}
