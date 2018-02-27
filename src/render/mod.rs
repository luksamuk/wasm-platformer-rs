use types::Vector2;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;


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
        js! { @{&self.ctx}.fillStyle = @{color}; } // Still needs to setup color manually
        self.ctx.fill_rect(pos.x, pos.y, size.x, size.y);
    }

    /// Draws a colored circle.
    pub fn draw_circle(&self, color: &str, pos: Vector2, radius: f64) {
        // Still needs to be done manually
        js! {
            @{&self.ctx}.beginPath();
            @{&self.ctx}.arc(@{pos.x}, @{pos.y}, @{radius}, 0, Math.PI * 2.0);
            @{&self.ctx}.fillStyle = @{color};
            @{&self.ctx}.fill();
            @{&self.ctx}.closePath();
        };
    }
}
