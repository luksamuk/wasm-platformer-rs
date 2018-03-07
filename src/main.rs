#![recursion_limit="2048"]

#[macro_use]
extern crate stdweb;
extern crate ref_eq;

use stdweb::unstable::TryInto;
use stdweb::traits::IMouseEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{
    self,
    IEventTarget,
    INonElementParentNode
};
use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    KeyDownEvent,
    KeyUpEvent,
    KeyboardLocation,

    MouseButton,
    MouseDownEvent,
    MouseUpEvent,
    MouseMoveEvent
};

use std::sync::Mutex;


#[macro_use]
pub mod common;    // Game objects, special println!, etc
pub mod types;     // Vectors, matrices, etc
pub mod collision; // Bounding volumes, collision, partitioning, etc
pub mod render;    // Rendering and etc
pub mod game;      // Actual specific game objects (Entity, etc)

use game::world::World;
use render::Renderer2D;


/// Handles keyboard events.
fn on_key(_key: &str, location: KeyboardLocation, _is_pressed: bool) -> bool {
    let _location = format!("{:?}", location);
    true
}


/// Handles mouse presses (up and down).
fn on_mouse_click(btn: MouseButton, _is_pressed: bool, _pos: (f64, f64)) -> bool {
    let _btn = format!("{:?}", btn);
    true
}

/// Handles sole mouse movement, without presses.
fn on_mouse_move(_pos: (f64, f64)) -> bool {
    true
}

fn semi_loop(mut world: World) {
    world.game_step();

    web::window().request_animation_frame(move |_| {
        semi_loop(world.clone());
    });
}



fn main() {
    stdweb::initialize();

    // Retrieve canvas
    let mut canvas: CanvasElement = web::document().get_element_by_id("viewport")
        .unwrap()
        .try_into()
        .unwrap();
    canvas.set_width(580);
    canvas.set_height(500);

    // Create renderer
    let renderer: Renderer2D = Renderer2D::new(&canvas);

    // Create world
    let mut world: World = World::new(renderer, 800.0);
    world.init();
    


    // === Event bindings ===
    
    // Keyboard
    web::window().add_event_listener(|event: KeyDownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: KeyUpEvent| {
        if on_key(&event.key(), event.location(), false) {
            event.prevent_default();
        }
    });


    
    // Mouse
    web::window().add_event_listener(|event: MouseDownEvent| {
        if on_mouse_click(event.button(), true, (event.client_x() as f64,
                                                 event.client_y() as f64)) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: MouseUpEvent| {
        if on_mouse_click(event.button(), false, (event.client_x() as f64,
                                                  event.client_y() as f64)) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: MouseMoveEvent| {
        if on_mouse_move((event.client_x() as f64, event.client_y() as f64)) {
            event.prevent_default();
        }
    });

    web::window().request_animation_frame(move |_| {
        semi_loop(world.clone());
    });
    
    stdweb::event_loop();
}
