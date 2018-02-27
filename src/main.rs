#![recursion_limit="2048"]

use std::cell::RefCell;

#[macro_use]
extern crate stdweb;
extern crate ref_eq;

use stdweb::unstable::TryInto;
use stdweb::traits::IMouseEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{
    self,
    IEventTarget,
    INonElementParentNode,
    CanvasRenderingContext2d
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


#[macro_use]
pub mod common;    // Game objects, special println!, etc
pub mod types;     // Vectors, matrices, etc
pub mod collision; // Bounding volumes, collision, partitioning, etc
pub mod render;    // Rendering and etc
pub mod game;      // Actual specific game objects (Entity, etc)

use types::Vector2;

use collision::partitioning::Quadtree;

use common::game_object::wrap_to_ref;
use common::game_object::GameObjectRef;

use render::{ draw_box, draw_circle };

use game::objects::Entity;


/// Handles keyboard events.
fn on_key(key: &str, location: KeyboardLocation, is_pressed: bool) -> bool {
    let location = format!("{:?}", location);
    //console!(log, "Key: ", key, ", location: ", location, ", pressed: ", is_pressed);
    true
}


/// Handles mouse presses (up and down).
fn on_mouse_click(btn: MouseButton, is_pressed: bool, pos: (f64, f64)) -> bool {
    let btn = format!("{:?}", btn);
    //console!(log, "MPos: (", pos.0, ", ", pos.1, ") MBtn: ", btn, " pressed: ", is_pressed);
    true
}

/// Handles sole mouse movement, without presses.
fn on_mouse_move(pos: (f64, f64)) -> bool {
    //console!(log, "MPos: (", pos.0, ", ", pos.1, ")");
    true
}



fn main() {
    stdweb::initialize();

    // Retrieve canvas
    let canvas: CanvasElement = web::document().get_element_by_id("viewport")
        .unwrap()
        .try_into()
        .unwrap();
    canvas.set_width(580);
    canvas.set_height(500);

    // Retrieve context
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();


    // === CONFIGURING COLLISION AND ADDING DUMMIES ==

    println!("Creating quadtree...");
    let mut my_quadtree: Quadtree<Entity> = Quadtree::new(Vector2::zero(), 400.0, 4);
    println!("Done.");


    // Test entities
    let mut entity_pos = vec![];
    let colors = vec!["#ff00007f", "#00ff007f", "#0000ff7f"];


    // Fixed entities
    entity_pos.push(Vector2 { x: 200.0, y: 200.0 });
    entity_pos.push(Vector2 { x: 300.0, y: 200.0 });
    entity_pos.push(Vector2 { x: 250.0, y: 250.0 });

    
    // Random entities
    let iterations = 4;
    for _ in 0..iterations
    {
        let pos = Vector2 {
            x: js!(return 100.0 + Math.floor((Math.random() * 300));).try_into().unwrap(),
            y: js!(return 100.0 + Math.floor((Math.random() * 300));).try_into().unwrap()
        };
        entity_pos.push(pos);
    }
    
    // Finally, add them
    {
        let mut color_idx = 0;
        for pos in entity_pos {
            println!("Adding entity at ({}, {})...", pos.x, pos.y);
            my_quadtree.add(wrap_to_ref(Entity::new(color_idx as u32, pos, colors[color_idx % 3], ctx.clone())));
            color_idx += 1;
        }
    }

    


    // === COLLISION PASS ===

    
    println!("Testing all collisions...");
    my_quadtree.test_collisions();
    println!("Done!");


    // === GAMELOOP ITERATION PASS ===

    println!("Iterating over all entities...");
    for object in my_quadtree.iter() {
        object.borrow_mut().update(0.0);
        object.borrow_mut().draw();
    }

    


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

    
    
    stdweb::event_loop();
}
