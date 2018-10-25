extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};

mod level;
use level::*;

pub fn main() {
    // glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Ops, I stepped a shit... G L F W");

    let (mut window, events) = glfw.create_window(300, 300, "Test", glfw::WindowMode::Windowed)
        .expect("Error on window creation");

    window.set_key_polling(true);
    window.make_current();

    window.show();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Level initialization
    let entity = Entity::new();

    let mut level = Level::new();
    level.spawn(entity);



    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        }

        level.update(1.0);

        // window update
        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                    println!("Bye bye");
                },
                glfw::WindowEvent::Key(Key::F, _, Action::Press, _) => {
                    println!("FPS: ...");
                },
                _ => {}
            }
        }
    }
}
