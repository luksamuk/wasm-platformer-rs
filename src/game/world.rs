use collision::partitioning::Quadtree;
use game::objects::Entity;
use render::Renderer2D;
use types::Vector2;
use stdweb::unstable::TryInto;

use common::objects::GameObject;
use common::objects::wrap_to_ref;

/// Represents a game world.
pub struct World {
    partitions: Quadtree<Entity>,
    renderer:   Renderer2D,
    running:    bool,
}

impl World {
    pub fn new(renderer: Renderer2D, world_max_size: f64) -> Self {
        World {
            // TODO: Figure out a better way to define depth
            partitions: Quadtree::new(Vector2::zero(), world_max_size / 2.0, 4),
            renderer:   renderer,
            running:    true,
        }
    }

    pub fn init(&mut self) {
        // Add test entities
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
                //println!("Adding entity at ({}, {})...", pos.x, pos.y);
                self.partitions.add(wrap_to_ref(Entity::new(color_idx as u32, pos, colors[color_idx % 3])));
                color_idx += 1;
            }
        }
    }

    pub fn game_loop(&mut self) {
        while self.running {
            // Test object collisions
            self.partitions.test_collisions();

            // Clear screen
            self.renderer.clear();

            // Iterating over all entities
            let iterator = self.partitions.iter();
            for object in iterator {
                let old_position = object.borrow().get_position();
                object.borrow_mut().update(0.0); // Update
                let new_position = object.borrow().get_position();
                
                object.borrow_mut().draw(&self.renderer); // Draw

                if old_position != new_position {
                    self.partitions.schedule_update(object.clone(), old_position);
                }
            }

            // Relocate moved objects
            let _ = self.partitions.update_positions();
        }
    }
}
