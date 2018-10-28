

use glutin_window::GlutinWindow;
use opengl_graphics::*;
use piston::input::*;
use std::cmp::min;
use animation::AnimationSystem;
use spritesheet::SpriteSheet;
use std::rc::Rc;
use find_folder;
use sprite::Sprite;
use spritesheet::SpriteAnimation;

pub struct Game {
    pub gl: GlGraphics,
    pub window: GlutinWindow,
    sprite: SpriteSheet
}

impl Game {
    pub fn new(window: GlutinWindow) -> Game {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let img = assets.join("paolo_utopinelli.png");
        let img = Texture::from_path(
            &img,
            &TextureSettings::new()
        ).unwrap();

        println!("{}, {}", img.get_width(), img.get_height());

        let mut sprite = SpriteSheet::new(Sprite::from_texture(Rc::new(img)), 0.25);
        sprite.add_animation(String::from("action"), SpriteAnimation {
            flip_x: false,
            flip_y: false,
            scale: 2.0,
            rect: [0.0, 0.0, 160.0, 96.0/2.0],
            cell_count: 5,
        });
        sprite.set_animation(String::from("action"));

        Game {
            gl: GlGraphics::new(OpenGL::V3_2),
            window,
            sprite,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let size = min(args.height, args.width) as f64 / 2.0;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        let sprite = &self.sprite;

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(WHITE, gl);

            let transform = ctx.transform.trans(x, y);
            //    .rot_rad(rotation)
            //   .trans(-size / 2.0, -size / 2.0);
            //rectangle(BLUE, square, transform, gl);

            sprite.draw(transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.sprite.on_time_pass(args.dt);
    }
}
