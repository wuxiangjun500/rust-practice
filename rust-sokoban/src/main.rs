use std::{collections::HashMap, iter, path};

use ggez::{
    conf, event, glam::Vec2, graphics::{self, DrawParam, Image}, input::keyboard::KeyCode, Context, GameResult
};
use hecs::{Entity, World};

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

struct Game {
    world: World,
}

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W 
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";
    laod_map(world, MAP.to_string());
}

pub fn laod_map(world: &mut World, map_string: String) {
    let rows:Vec<&str> = map_string.trim().split("\n").map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0
            };
            match *column {
                "." => {
                    create_floor(world, position);
                },
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                },
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                },
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                },
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                },
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[allow(dead_code)]
pub struct Renderable {
    path: String,
}

pub struct Movable;

pub struct Immovable;

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
        Immovable {},
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
        Movable {},
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
        Movable {},
    ))
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

pub fn run_input(world: &World, _ctx: &mut Context) {
    let mut to_move: Vec<(Entity, KeyCode)> = Vec::new();

    let mov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Movable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect::<HashMap<_, _>>();
    let immov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Immovable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect::<HashMap<_, _>>();

    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if _ctx.keyboard.is_key_repeated() {
            continue;
        }
        let key = if _ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            KeyCode::Up
        } else if _ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            KeyCode::Down
        } else if _ctx.keyboard.is_key_just_pressed(KeyCode::Left) {
            KeyCode::Left
        } else if _ctx.keyboard.is_key_just_pressed(KeyCode::Right) {
            KeyCode::Right
        } else {
            continue;
        };
        
        let (start, end , is_x) = match key {
            KeyCode::Up => (position.y, 0, false),
            KeyCode::Down => (position.y, MAP_HEIGHT - 1, false),
            KeyCode::Left => (position.x, 0, true),
            KeyCode::Right => (position.x, MAP_WIDTH, true),
            _ => continue,
        };

        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };
        for x_or_y in range {
            let pos = if is_x {
                (x_or_y, position.y)
            } else {
                (position.x, x_or_y)
            };
            match mov.get(&pos) {
                Some(entity) => to_move.push((*entity, key)),
                None => {
                    match immov.get(&pos) {
                        Some(_id) => to_move.clear(),
                        None => break,
                    }
                },
            }
        }
    }

    for (entity, key) in to_move {
        let mut position = world.get::<&mut Position>(entity).unwrap();
        match key {
            KeyCode::Up => position.y -= 1,
            KeyCode::Down => position.y += 1,
            KeyCode::Left => position.x -= 1,
            KeyCode::Right => position.x += 1,
            _ => (),
        }
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

#[allow(dead_code)]
fn run_system_duplicat(world: &World, _ctx: &mut Context) {
    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            position.y -= 1;
        }
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            position.y += 1;
        }
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Left) {
            position.x -= 1;
        }
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Right) {
            position.x += 1;
        }
    }
}

#[allow(dead_code)]
fn run_input_print(_world: &World, context: &mut Context) {
    if context.keyboard.is_key_pressed(KeyCode::Up) {
        println!("UP");
    }
    if context.keyboard.is_key_pressed(KeyCode::Down) {
        println!("DOWN");
    }
    if context.keyboard.is_key_pressed(KeyCode::Left) {
        println!("LEFT");
    }
    if context.keyboard.is_key_pressed(KeyCode::Right) {
        println!("RIGHT");
    }
}