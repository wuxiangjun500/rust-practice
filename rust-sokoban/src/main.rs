use std::path;

use ggez::{
    conf, event, GameResult
};
use hecs::World;
use map::initialize_level;
use systems::{input::run_input, rendering::run_rendering};

mod components;
mod constants;
mod entities;
mod map;
mod systems;

struct Game {
    world: World,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        {
            run_input(&self.world, _ctx);
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        {
            run_rendering(&self.world, _ctx);
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mut world = World::new();
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./rust-sokoban/resources"));
    let (context, event_loop) = context_builder.build()?;

    let game = Game { world };

    event::run(context, event_loop, game)
}