use amethyst::core::Transform;

use amethyst::ecs::{Read, ReadStorage, WriteStorage};
use amethyst::ecs::{System, Entity};

use amethyst::input::InputHandler;

pub mod hgame;

pub struct PlayerMovement;

impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, hgame::Human>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, humans, input): Self::SystemData) {
        println!("Looping");
        //let transform = transforms.get_mut(self.player).unwrap();
        //transform.translation.y += 0.1;
        //println!("player moved to x={},y={}", transform.translation.x, transform.translation.y)
    }
}
