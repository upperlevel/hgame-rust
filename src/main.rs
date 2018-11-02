extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage};
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;

mod systems;
use systems::hgame;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new())
    );

    let input_handle = InputBundle::<String, String>::new()
        .with_bindings_from_file("resources/bindings_config.ron")?;

    let game_data = GameDataBuilder::default()
        // a bundle is a preset of systems and resources
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())? // this bundle will also initialise the transform storage on world.
        .with_bundle(input_handle)?
        .with(systems::PlayerMovement, "player_movement_system", &["input_system"]);

    let mut game = Application::new("./resources", hgame::HGame, game_data)?;
    game.run();

    Ok(())
}
