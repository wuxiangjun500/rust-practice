use std::path;

use ggez::{
    conf, event, glam::Vec2, graphics::{self, DrawParam, Image}, Context, GameResult
};
use hecs::{Entity, World};

const TILE_WIDTH: f32 = 32.0;

struct Game {
    world: World,
}

pub fn initialize_level(world: &mut World) {
    create_player(
        world,
        Position {
            x: 0,
            y: 0,
            z: 0, // we will get the z from the factory functions
        },
    );
    create_wall(
        world,
        Position {
            x: 1,
            y: 0,
            z: 0, // we will get the z from the factory functions
        },
    );
    create_box(
        world,
        Position {
            x: 2,
            y: 0,
            z: 0, // we will get the z from the factory functions
        },
    );
}

#[allow(dead_code)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[allow(dead_code)]
pub struct Renderable {
    path: String,
}

pub struct Wall {}

pub struct Player {}

pub struct Box {}

pub struct BoxSpot {}

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/wall.png".to_string(),
        },
        Wall {},
    ))
}

pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 5, ..position },
        Renderable {
            path: "/images/floor.png".to_string(),
        },
    ))
}

pub fn create_box(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/box.png".to_string(),
        },
        Box {},
    ))
}

pub fn create_box_spot(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable {
            path: "/images/box_spot.png".to_string(),
        },
        BoxSpot {},
    ))
}

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/player.png".to_string(),
        },
        Player {},
    ))
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
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

pub fn run_rendering(world: &World, context: &mut Context) {
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));
    let mut query = world.query::<(&Position, &Renderable)>();

    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    for (_, (position, renderable)) in rendering_data.iter() {
        let image = Image::from_path(context, renderable.path.clone()).unwrap();
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        canvas.draw(&image, draw_params);
    }

    canvas.finish(context).expect("expected to present");
}

