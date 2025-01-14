use std::collections::HashMap;

use ggez::{input::keyboard::KeyCode, Context};
use hecs::{Entity, World};

use crate::{components::{Immovable, Movable, Player, Position}, constants::{MAP_HEIGHT, MAP_WIDTH}};


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