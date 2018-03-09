use types::Vector2;
use stdweb::web::html_element::ImageElement;
use render::Renderer2D;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Texture {
    data: ImageElement,
}

impl Texture {
    pub fn from(source: &str) -> Self {
        let data = ImageElement::new();
        data.set_src(source);
        Texture {
            data: data,
        }
    }

    fn get_data(&self) -> ImageElement {
        self.data.clone()
    }

    pub fn draw(&self, renderer: &Renderer2D, pos: Vector2) {
        renderer.draw_image_rel(self.data.clone(), pos);
    }
}


#[derive(Clone)]
pub struct TextureAtlas {
    texture:   Texture,
    tile_size: Vector2,
}

impl TextureAtlas {
    pub fn from(source: &str, tile_size: Vector2) -> Self {
        TextureAtlas {
            texture: Texture::from(source),
            tile_size: tile_size,
        }
    }

    pub fn draw_frame(&self, renderer: &Renderer2D, pos: Vector2, frame: u32) {
        renderer.draw_tile_rel(self.texture.get_data(), pos, self.tile_size, frame);
    }

    pub fn draw_all(&self, renderer: &Renderer2D, pos: Vector2) {
        renderer.draw_image_rel(self.texture.get_data(), pos);
    }
}


type AnimationMap = HashMap<String, (f64, Vec<u32>)>;

#[derive(Clone)]
pub struct Animator {
    animations:    AnimationMap,
    current_anim:  Option<String>,
    current_frame: u32,
    current_speed: f64,
    current_data:  Option<Vec<u32>>,
    time_accum:    f64,
}

impl Animator {
    pub fn new() -> Self {
        Animator {
            animations:    HashMap::new(),
            current_anim:  None,
            current_frame: 0,
            current_speed: 0.0,
            current_data:  None,
            time_accum:    0.0,
        }
    }

    pub fn register(&mut self, name: &str, default_spd: f64, data: &[u32]) {
        self.animations.insert(String::from(name),
                               (default_spd, Vec::from(data)));
        println!("Registered {} with {} frames", name, data.len());
    }

    pub fn update(&mut self, dt: f64) {
        // Verify if we're actually animating
        if self.current_anim.is_some() {
            // Increase time accumulator
            self.time_accum += dt;

            // If we exceeded the amount of time we can
            // pass on a single frame, then we need to change it
            if self.time_accum >= self.current_speed {
                // Reset timer and calculate our frames skip
                let frames_passed =
                    (self.time_accum / self.current_speed).floor() as u32;
                self.time_accum -= frames_passed as f64 * self.current_speed;

                // We take a peek at our animation data so we
                // can wrap around frames
                match self.current_data {
                    Some(ref data) => {
                        // Determine current frame
                        self.current_frame += frames_passed;
                        self.current_frame %= data.len() as u32;
                        //println!("Update: frame is {}", self.current_frame);
                    },
                    None => {
                        // Uhhh this is unexpected.
                        panic!("Unexpected animation with no frame data");
                    },
                }
            }
        }
    }

    pub fn draw(&self, renderer: &Renderer2D, pos: Vector2, atlas: &TextureAtlas) {
        if self.current_anim.is_some() {
            match self.current_data {
                Some(ref data) => {
                    let current_frame = data[self.current_frame as usize];
                    atlas.draw_frame(renderer, pos, current_frame);
                },
                None => {},
            }
        }
    }

    pub fn set_animation(&mut self, name: &str) {
        let current_anim = self.current_anim.clone();
        match current_anim {
            Some(ref anim_name) => {
                if anim_name.as_str() != name {
                    // Test whether animation is valid
                    match self.animations.get(&String::from(name)) {
                        Some(ref pair) => {
                            // We actually change the animation now
                            self.current_anim = Some(String::from(name));
                            self.current_frame = 0;
                            self.current_speed = pair.0;
                            self.current_data  = Some(pair.1.clone());
                            self.time_accum = 0.0;
                        },
                        None => {println!("Couldn't find animation \"{}\".", name);},
                    }
                }
            },
            None => {
                match self.animations.get(&String::from(name)) {
                    Some(ref pair) => {
                        // We actually change the animation now
                        self.current_anim = Some(String::from(name));
                        self.current_frame = 0;
                        self.current_speed = pair.0;
                        self.current_data  = Some(pair.1.clone());
                        self.time_accum = 0.0;
                    },
                    None => {println!("Couldn't find animation \"{}\".", name);},
                }
            },
        }
    }
}
