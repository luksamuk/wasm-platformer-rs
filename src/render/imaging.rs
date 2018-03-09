use types::Vector2;
use stdweb::web::html_element::ImageElement;
use render::Renderer2D;

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
}
