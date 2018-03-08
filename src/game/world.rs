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
    camera:     Camera,
}

impl World {
    /// Yields a new instance of a World.
    pub fn new(renderer: Renderer2D, world_max_size: f64) -> Self {
        World {
            // TODO: Figure out a better way to define depth
            partitions: wrap_to_ref(Quadtree::new(Vector2::zero(), world_max_size / 2.0, 4)),
            renderer:   renderer,
            running:    true,
            camera:     Camera::new(Vector2::new(640.0, 360.0))
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

        // Add the player
        {
            let player = wrap_to_ref(
                Entity::new(0, Vector2::zero(), "#0000007f"));
            self.partitions.borrow_mut().add(player.clone());
            // Make camera follow it
            self.camera.follow(Some(player));
        }

        // Finally, add them
        {
            let mut color_idx = 1;
            for pos in entity_pos {
                println!("Adding entity at ({}, {})...", pos.x, pos.y);
                self.partitions.borrow_mut().add(wrap_to_ref(Entity::new(color_idx as u32, pos, colors[color_idx % 3])));
                color_idx += 1;
            }
        }

        // Set camera position
        self.camera.translate(Vector2::new(320.0, 180.0));
    }

    /// Executes a step in the World, updating logic, rendering and collision.
    pub fn game_step(&mut self, dt: f64) {
        // == Collision pass == //
        self.partitions.borrow_mut().test_collisions();

        // == Clear screen pass == //
        self.renderer.clear();

        // == Camera update pass == //
        self.camera.update();
        let local_camera_boundary = self.camera.bounding_circle();
        self.renderer.update_camera_position({
            local_camera_boundary.center - self.camera.half_viewport_size()
        });

        // == Object iteration pass == //
        let iterator = self.partitions.borrow().local_iter(local_camera_boundary);
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
        self.camera.update();
    }
}


// =============================

use collision::primitives::Delimitable;
use collision::primitives::AABB;
use collision::primitives::Circle;

#[derive(Clone)]
pub struct Camera {
    viewport: AABB,
    follows:  Option<ObjectRef<GameObject>>,
}

impl Camera {
    fn new(size: Vector2) -> Self {
        Camera {
            viewport: AABB {
                center: Vector2::zero(),
                halfws: [size.x / 2.0, size.y / 2.0],
            },
            follows: None,
        }
    }

    fn update(&mut self) {
        match self.follows {
            Some(ref guy) => {
                // TODO: This simple attribution looks like crap!
                // We need to properly recalculate the camera position,
                // just like in Sonic The Hedgehog games.
                self.viewport.center = guy.borrow().get_position();
            },
            None => {},
        }
    }

    fn translate(&mut self, position: Vector2) {
        self.viewport.center = position;
    }

    fn follow(&mut self, object: Option<ObjectRef<GameObject>>) {
        self.follows = object;
    }

    fn viewport_size(&self) -> Vector2 {
        Vector2::new(self.viewport.halfws[0] * 2.0,
                     self.viewport.halfws[1] * 2.0)
    }

    fn half_viewport_size(&self) -> Vector2 {
        Vector2::new(self.viewport.halfws[0],
                     self.viewport.halfws[1])
    }
}

impl Delimitable for Camera {
    fn bounding_circle(&self) -> Circle {
        let mut bounds = self.viewport.bounding_circle();
        bounds.radius *= 2.0;
        bounds
    }
}

