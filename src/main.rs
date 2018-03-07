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


#[macro_use]
pub mod common;    // Game objects, special println!, etc
pub mod types;     // Vectors, matrices, etc
pub mod collision; // Bounding volumes, collision, partitioning, etc
pub mod render;    // Rendering and etc
pub mod game;      // Actual specific game objects (Entity, etc)

use types::Vector2;
use collision::partitioning::Quadtree;
use common::objects::GameObject;
use common::objects::wrap_to_ref;
use game::objects::Entity;
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



fn main() {
    stdweb::initialize();

    // Retrieve canvas
    let canvas: CanvasElement = web::document().get_element_by_id("viewport")
        .unwrap()
        .try_into()
        .unwrap();
    canvas.set_width(580);
    canvas.set_height(500);

    // Create renderer
    let renderer = Renderer2D::new(&canvas);

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
            my_quadtree.add(wrap_to_ref(Entity::new(color_idx as u32, pos, colors[color_idx % 3])));
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
        let old_position = object.borrow().get_position();
        object.borrow_mut().update(0.0);
        let new_position = object.borrow().get_position();
        
        object.borrow_mut().draw(&renderer);

        if old_position != new_position {
            println!("Scheduling object relocation");
            my_quadtree.schedule_update(object.clone(), old_position);
        }
    }

    println!("Relocating updated objects...");
    let _ = my_quadtree.update_positions();

    renderer.clear();

    println!("Testing all collisions...");
    my_quadtree.test_collisions();
    println!("Done!");

    println!("Iterating over all entities...");
    for object in my_quadtree.iter() {
        let old_position = object.borrow().get_position();
        object.borrow_mut().update(0.0);
        let new_position = object.borrow().get_position();
        
        object.borrow_mut().draw(&renderer);

        if old_position != new_position {
            console!(log, "Scheduling object relocation");
            my_quadtree.schedule_update(object.clone(), old_position);
        }
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
