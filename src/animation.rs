use std::collections::HashMap;
use std::rc::Rc;
use spritesheet::SpriteSheet;
use std::cell::RefCell;
use sprite::Sprite;
use opengl_graphics::{Texture, TextureSettings};

pub struct AnimationSystem {
    sheets: HashMap<String, Rc<RefCell<SpriteSheet>>>
}

impl AnimationSystem {
    pub fn new() -> AnimationSystem {
        AnimationSystem {
            sheets: HashMap::new()
        }
    }

    pub fn update(&mut self, dt: f64) {
        for sheet in self.sheets.values_mut() {
            sheet.borrow_mut().on_time_pass(dt);
        }
    }

    fn load_tex(name: &str) -> Result<Texture, String> {
        Texture::from_path(name, &TextureSettings::new())
    }

    pub fn create(path: String, frame_time: f64) -> Result<SpriteSheet, String> {
        let tex = AnimationSystem::load_tex(&path)?;
        let spritesheet = SpriteSheet::new(Sprite::from_texture(Rc::new(tex)), frame_time);
        Ok(spritesheet)
    }
}

