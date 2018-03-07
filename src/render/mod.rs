use types::Vector2;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;
use std::f64::consts::PI;


/// Represents a 2D renderer.
pub struct Renderer2D {
    ctx: CanvasRenderingContext2d,
}


impl Renderer2D {
    /// Constructor for 2D renderer.
    /// # Arguments
    /// `canvas` - Reference to the canvas element where the context lives.
    pub fn new(canvas: &CanvasElement) -> Self {
        Renderer2D {
            ctx: canvas.get_context().unwrap(),
        }
    }
    
    /// Draws a colored box.
    pub fn draw_box(&self, color: &str, pos: Vector2, size: Vector2) {
        self.ctx.set_fill_style_color(color);
        self.ctx.fill_rect(pos.x, pos.y, size.x, size.y);
    }

    /// Draws a colored circle.
    pub fn draw_circle(&self, color: &str, pos: Vector2, radius: f64) {
        self.ctx.begin_path();
        self.ctx.set_fill_style_color(color);
        self.ctx.arc(pos.x, pos.y, radius, 0.0, PI * 2.0, false);
        js!{ @(no_return) @{&self.ctx}.fill(); };
        self.ctx.close_path();
    }
}
