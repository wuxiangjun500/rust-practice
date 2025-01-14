use ggez::{glam::Vec2, graphics::{self, DrawParam, Image}, input::keyboard::KeyCode, Context};
use hecs::{Entity, World};

use crate::{components::{Player, Position, Renderable}, constants::TILE_WIDTH};

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