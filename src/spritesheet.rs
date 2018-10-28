use opengl_graphics::GlGraphics;
use graphics::math::Matrix2d;

use opengl_graphics::*;
use graphics::DrawState;
use graphics::types::SourceRectangle;
use graphics::math::{Vec2d, Scalar};
use sprite::Sprite;
use std::collections::HashMap;
use graphics::ImageSize;
use graphics::Graphics;
use opengl_graphics::Texture;
use std::cell::RefCell;

pub struct SpriteSheet {
    sprite: RefCell<Sprite<Texture>>,
    animations: HashMap<String, SpriteAnimation>,
    frame_time: f64,
    current_animation: String,
    current_index: usize,
    current_time: f64,
}

pub struct SpriteAnimation {
    pub flip_x: bool,
    pub flip_y: bool,
    pub scale: Scalar,
    pub rect: SourceRectangle,
    pub cell_count: usize,
}

impl SpriteSheet {
    pub fn new(sprite: Sprite<Texture>, frame_time: f64) -> SpriteSheet {
        SpriteSheet {
            sprite: RefCell::new(sprite),
            animations: HashMap::new(),
            frame_time,
            current_animation: String::new(),
            current_index: 0,
            current_time: 0.0,
        }
    }

    pub fn get_current_animation(&self) -> &SpriteAnimation {
        self.animations.get(&self.current_animation).unwrap()
    }

    pub fn on_time_pass(&mut self, time: f64) {
        self.current_time += time;
        if self.current_time >= self.frame_time {
            self.current_time -= self.frame_time;
            self.current_index += 1;
            let cell_count = self.get_current_animation().cell_count;
            if self.current_index >= cell_count {
                self.current_index = 0;
            }
        }
    }

    pub fn add_animation(&mut self, name: String, animation: SpriteAnimation) {
        self.animations.insert( name, animation);
    }

    pub fn set_animation(&mut self, name: String)  {
        self.current_animation = name;
    }

    pub fn draw(&self, trasform: Matrix2d, graphics: &mut GlGraphics) {
        let animation = self.animations.get(&self.current_animation).expect("Cannot find current animation");

        let mut sprite = self.sprite.borrow_mut();

        sprite.set_flip_x(animation.flip_x);
        sprite.set_flip_y(animation.flip_x);

        let src_rect = animation.rect;

        let cell_w = (src_rect[2] - src_rect[0]) / animation.cell_count as f64;
        let cell_index = self.current_index as f64;


        let dst_rect = [
            src_rect[0] + cell_w * cell_index, src_rect[1],
            cell_w, src_rect[3],
        ];

        sprite.set_src_rect(dst_rect);
        sprite.draw(trasform, graphics);
    }
}