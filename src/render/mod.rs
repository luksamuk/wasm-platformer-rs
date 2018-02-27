use types::Vector2;
use stdweb::web::CanvasRenderingContext2d;


/// Draws a colored box.
pub fn draw_box(ctx: &CanvasRenderingContext2d, color: &str, pos: Vector2, size: Vector2) {
    js! { @{ctx}.fillStyle = @{color}; } // Still needs to setup color manually
    ctx.fill_rect(pos.x, pos.y, size.x, size.y);
}

/// Draws a colored circle.
pub fn draw_circle(ctx: &CanvasRenderingContext2d, color: &str, pos: Vector2, radius: f64) {
    // Still needs to be done manually
    js! {
        @{ctx}.beginPath();
        @{ctx}.arc(@{pos.x}, @{pos.y}, @{radius}, 0, Math.PI * 2.0);
        @{ctx}.fillStyle = @{color};
        @{ctx}.fill();
        @{ctx}.closePath();
    };
}
