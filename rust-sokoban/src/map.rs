use hecs::World;

use crate::{components::Position, entities::{create_box, create_box_spot, create_floor, create_player, create_wall}};

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
