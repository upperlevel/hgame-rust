extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate glutin_window;
extern crate sprite;
extern crate find_folder;

use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::window::WindowSettings;
use piston::input::*;

mod game;
use game::Game;

mod spritesheet;
mod animation;

use glutin_window::GlutinWindow;
use std::io;
use std::io::Write;

pub fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Piston window.
    let mut window: GlutinWindow = WindowSettings::new(
        "hgame",
        [200, 200],
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game::new(window);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut game.window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.press_args() {
            println!("Press: {:?}", args);
            io::stdout().flush().unwrap();
        }
    }
}
