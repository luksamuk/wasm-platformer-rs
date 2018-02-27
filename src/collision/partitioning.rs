//! Submodule related to spatial partitioning.

use types::Vector2;
use std::rc::Rc;
use std::cell::RefCell;
use common::game_object::{ GameObject, ObjectRef };
use collision::primitives::{ Circle, Collidable };

type QuadtreeNodeCountedRef<T> = Rc<RefCell<QuadtreeNode<T>>>;

#[derive(Clone)]
#[repr(C)]
struct QuadtreeNode<T: GameObject> {
    center:   Vector2,
    halfw:    f64,
    children: Vec<QuadtreeNodeCountedRef<T>>,
    objects:  Vec<ObjectRef<T>>,
}

impl<T: GameObject> QuadtreeNode<T> {
    fn add(&mut self, object: ObjectRef<T>) {
        let mut index: usize = 0;
        let mut straddle = false;
        let bounds = object.borrow().bounding_circle();
        // Compute quadrant number [0..4] that the object circle center is in.
        // If straddling any of the dividing x or y planes, exit directly
        let delta = bounds.center - self.center;

        // Check for X
        if delta.x.abs() > bounds.radius {
            if delta.x > 0.0 { index |= 1; }
            // Check for Y
            if delta.y.abs() > bounds.radius {
                if delta.y > 0.0 { index |= 2; }
            } else { straddle = true; }
        } else { straddle = true; }

        if !straddle && !self.children.is_empty() {
            // Fully contained in existing child node! Insert in subtree.
            //println!("DEBUG@QuadtreeNode::add: descent subtree #{}", index + 1);
            self.children[index].borrow_mut().add(object);
        } else {
            // Straddling or no child node to descend to.
            // Link object into linked list at this node.
            //println!("DEBUG@QuadtreeNode::add: add object to quadtree. Reason: {}",
            //         if self.children.is_empty() { "depth limit" } else { "straddling" });
            self.objects.push(object);
        }
    }
}



/// Implements a quadtree, a special tree which partitions the entire space in
/// four blocks. Each block is then recursively divided like the former space,
/// until the tree reaches its depth limit.
pub struct Quadtree<T: GameObject> {
    root:      QuadtreeNodeCountedRef<T>,
    max_depth: u32,

    // For iteration purposes only!
    // Must always have len() zero after an iteration.
    ancestors: Vec<Option<QuadtreeNodeCountedRef<T>>>,
}

// Constructor
impl<T: GameObject> Quadtree<T> {
    /// Creates a new quadtree.
    /// # Arguments
    /// `center` - Center of space to be partitioned.
    /// `half_width` - Half-width of space to be partitioned. Remember that the quadtree
    /// assumes a squared space, not a rectangular one.
    /// `max-depth` - Maximum depth the quadtree can reach. If no depth is provided,
    /// the tree will only divide the space in four areas. A depth of three should be fine
    /// for simple cases.
    pub fn new(center: Vector2, half_width: f64, max_depth: u32) -> Quadtree<T> {
        Quadtree {
            root: Quadtree::build_tree(center, half_width, max_depth + 1).unwrap(),
            max_depth: max_depth,
            ancestors: vec![],
        }
    }
    
    fn build_tree(center: Vector2, half_width: f64, stop_depth: u32) -> Option<QuadtreeNodeCountedRef<T>> {
        match stop_depth {
            0 => None,
            // Construct and fill the root of this subtree
            _ => Some(Rc::new(RefCell::new(QuadtreeNode {
                    center:   center,
                    halfw:    half_width,
                    objects:  Vec::new(),
                    children: {
                        let mut children = vec![];
                        let step = half_width * 0.5;
                        for i in 0..4 {
                            let offset_x = if i & 1 != 0 { step } else { -step };
                            let offset_y = if i & 2 != 0 { step } else { -step };
                            let offset = Vector2::new(offset_x, offset_y);
                            if let Some(node) = Quadtree::build_tree(center + offset, step, stop_depth - 1) {
                                children.push(node);
                            }
                        }
                        
                        children
                    },
            }))),
        }
    }

    /// Yields an iterator for this quadtree.
    pub fn iter(&self) -> QuadtreeIter<T> {
        QuadtreeIter {
            nodes:   vec![QuadtreeIterNode {
                current: 0,
                read: false,
                node: self.root.clone() }],
        }
    }
}


// General methods
impl<T: 'static + GameObject> Quadtree<T> {
    /// Adds a game object to the quadtree.
    /// # Arguments
    /// `object` - A dynamically-allocated object which should be added to spatial
    /// partitioning structure.
    pub fn add(&mut self, object: ObjectRef<T>) {
        self.root.borrow_mut().add(object);
    }

    /// Tests all collisions between objects, calling
    /// each object's `on_collision` callback.
    pub fn test_collisions(&mut self) {
        self.test_all_collisions(None);

        // "muh asserts on production"
        // yep, son
        assert!(self.ancestors.len() == 0);
    }

    fn test_all_collisions(&mut self, tree: Option<QuadtreeNodeCountedRef<T>>) {
        // Keep track of ancestor objects
        self.ancestors.push(tree.clone());
        
        for root_a in &self.ancestors {
            //let root_a = &self.ancestors[n];
            let root_a = match root_a {
                &None => self.root.clone(),
                &Some(ref node) => node.clone(),
            };

            let root_b = match &tree {
                &None => self.root.clone(),
                &Some(ref node) => node.clone(),
            };

            // Only traded places between names obj_a and obj_b
            // to make outputs and comparision order prettier.
            // Bear with me.
            for obj_b in root_a.borrow().objects.iter() {
                for obj_a in root_b.borrow().objects.iter() {
                    use ref_eq::ref_eq;

                    // Discard same objects
                    if ref_eq(&*obj_a, &*obj_b) {
                        break;
                    } // Does this really work???
 
                    // Test collision
                    let bounding_a = (**obj_a).borrow().bounding_circle();
                    let bounding_b = (**obj_b).borrow().bounding_circle();

                    if bounding_a.collides(&bounding_b) {
                        // Trigger collision callbacks and pass along
                        // a clone of the counted reference
                        obj_a.borrow_mut().on_collision((*obj_b).clone());
                        obj_b.borrow_mut().on_collision((*obj_a).clone());
                    }
                }
            }
        }

        let children_root = tree.unwrap_or(self.root.clone());
        let children = &children_root.borrow().children;
        for node in children {
            self.test_all_collisions(Some(node.clone()));
        }

        // Remove current node from ancestor stack before returning
        let _ = self.ancestors.pop();
    }
}

struct QuadtreeIterNode<T: GameObject> {
    current: u32,
    read:    bool,
    node:    QuadtreeNodeCountedRef<T>,
}

/// Iterator for Quadtree.
/// Iterates on all objects on quadtree, though it doesn't guarantee
/// an iteration order.
pub struct QuadtreeIter<T: GameObject> {
    nodes:   Vec<QuadtreeIterNode<T>>,
}

impl<T: 'static + GameObject> Iterator for QuadtreeIter<T> {
    type Item = ObjectRef<GameObject>;

    fn next(&mut self) -> Option<ObjectRef<GameObject>> {
        // If not already iterated over, add all children
        // to read queue.
        let was_read = self.nodes.last()?.read; // Rets `None` when there is nobody else to read
        if !was_read {
            self.nodes.last_mut().unwrap().read = true;
            let node = self.nodes.last().unwrap().node.clone();
            for node in &node.borrow().children {
                self.nodes.push(QuadtreeIterNode {
                    current: 0,
                    read:    false,
                    node:    node.clone(),
                });
            }
        } else {
            let objects_len = self.nodes.last().unwrap().node.borrow().objects.len();
            let current = self.nodes.last().unwrap().current;
            if (current as usize) < objects_len {
                self.nodes.last_mut().unwrap().current += 1;

                // Since we iter over objects, then objects are the
                // only thing we Ret inside an Option.
                return Some(self.nodes.last().unwrap()
                            .node.borrow().objects[current as usize]
                            .clone());
            } else {
                // Remove myself from queue
                self.nodes.pop();
            }
        }
        // It's never enough tail recursions
        self.next()
    }
}
