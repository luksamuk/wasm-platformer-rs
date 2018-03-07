use collision::partitioning::Quadtree;
use game::objects::Entity;
use render::Renderer2D;
use types::Vector2;
use stdweb::unstable::TryInto;

use common::objects::GameObject;
use common::objects::ObjectRef;
use common::objects::wrap_to_ref;

/// Represents a game world.
#[derive(Clone)]
pub struct World {
    partitions: ObjectRef<Quadtree<Entity>>,
    renderer:   Renderer2D,
    running:    bool,
}

impl World {
    /// Yields a new instance of a World.
    pub fn new(renderer: Renderer2D, world_max_size: f64) -> Self {
        World {
            // TODO: Figure out a better way to define depth
            partitions: wrap_to_ref(Quadtree::new(Vector2::zero(), world_max_size / 2.0, 4)),
            renderer:   renderer,
            running:    true,
        }
    }

    /// Initializes the world.
    /// Currently, this serves only debugging purposes.
    pub fn init(&mut self) {
        // Add test entities
        let mut entity_pos = vec![];
        let colors = vec!["#ff00007f", "#00ff007f", "#0000ff7f"];


        // Fixed entities
        entity_pos.push(Vector2 { x: 200.0, y: 200.0 });
        entity_pos.push(Vector2 { x: 300.0, y: 200.0 });
        entity_pos.push(Vector2 { x: 250.0, y: 250.0 });

        
        // Random entities
        let iterations = 16;
        for _ in 0..iterations
        {
            let pos = Vector2 {
                x: js!(return 100.0 + Math.floor((Math.random() * 400));).try_into().unwrap(),
                y: js!(return 100.0 + Math.floor((Math.random() * 400));).try_into().unwrap()
            };
            entity_pos.push(pos);
        }

        // Finally, add them
        {
            let mut color_idx = 0;
            for pos in entity_pos {
                println!("Adding entity at ({}, {})...", pos.x, pos.y);
                self.partitions.borrow_mut().add(wrap_to_ref(Entity::new(color_idx as u32, pos, colors[color_idx % 3])));
                color_idx += 1;
            }
        }
    }

    /// Executes a step in the World, updating logic, rendering and collision.
    pub fn game_step(&mut self, dt: f64) {
        // == Collision pass == //
        self.partitions.borrow_mut().test_collisions();

        // == Clear screen pass == //
        self.renderer.clear();

        // == Object iteration pass == //
        let iterator = self.partitions.borrow().iter();
        for object in iterator {
            //println!("Render one");
            // == Update pass == //
            let old_position = object.borrow().get_position();
            object.borrow_mut().update(dt);
            let new_position = object.borrow().get_position();

            // == Draw pass == //
            object.borrow_mut().draw(&self.renderer);

            // Relocation scheduling
            if old_position != new_position {
                self.partitions.borrow_mut().schedule_update(object.clone(), old_position);
            }


        }

        // == Relocation pass == //
        let _ = self.partitions.borrow_mut().update_positions();
    }
}
