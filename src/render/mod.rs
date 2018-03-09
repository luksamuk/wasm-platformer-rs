use types::Vector2;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;
use std::f64::consts::PI;


/// Represents a 2D renderer.
#[derive(Clone)]
pub struct Renderer2D {
    ctx:        CanvasRenderingContext2d,
    sz:         Vector2,
    camera_pos: Vector2,
}


impl Renderer2D {
    /// Constructor for 2D renderer.
    /// # Arguments
    /// `canvas` - Reference to the canvas element where the context lives.
    pub fn new(canvas: &CanvasElement) -> Self {
        Renderer2D {
            ctx: canvas.get_context().unwrap(),
            sz:  Vector2 { x: canvas.width() as f64, y: canvas.height() as f64 },
            camera_pos: Vector2::zero(),
        }
    }

    pub fn update_camera_position(&mut self, new_position: Vector2) {
        self.camera_pos = new_position;
    }

    pub fn make_position_relative(&self, absolute_pos: Vector2) -> Vector2 {
        absolute_pos - self.camera_pos
    }
    
    /// Draws a colored box.
    pub fn draw_box(&self, color: &str, pos: Vector2, size: Vector2) {
        self.ctx.set_fill_style_color(color);
        self.ctx.fill_rect(pos.x, pos.y, size.x, size.y);
    }

    pub fn draw_box_rel(&self, color: &str, pos: Vector2, size: Vector2) {
        let pos = self.make_position_relative(pos);
        self.draw_box(color, pos, size);
    }


    /// Draws a colored circle.
    pub fn draw_circle(&self, color: &str, pos: Vector2, radius: f64) {
        self.ctx.begin_path();
        self.ctx.set_fill_style_color(color);
        self.ctx.arc(pos.x, pos.y, radius, 0.0, PI * 2.0, false);
        js!{ @(no_return) @{&self.ctx}.fill(); };
        self.ctx.close_path();
    }

    pub fn draw_circle_rel(&self, color: &str, pos: Vector2, radius: f64) {
        let pos = self.make_position_relative(pos);
        self.draw_circle(color, pos, radius);
    }
    
    /// Clears the screen.
    pub fn clear(&self) {
        self.ctx.clear_rect(0.0, 0.0, self.sz.x, self.sz.y);
    }
}
