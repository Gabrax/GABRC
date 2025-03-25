use std::fs;
use std::path::Path;

use raylib::prelude::*;

use crate::Player;

#[derive(Clone)]
pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub vx: f64, // Velocity in X direction
    pub vy: f64, // Velocity in Y direction
    pub dir_x: f64, // Velocity in Y direction
    pub dir_y: f64, // Velocity in Y direction
    pub is_projectile: f64,
    pub is_destroyed: f64,
    pub texture: i32,
}

#[derive(Clone)]
pub struct GameMap {
    pub size: usize,
    pub map_data: Vec<u8>,
    pub sprites: Vec<Sprite>,
}

impl GameMap {
    pub fn load_map(file_path: &str) -> Self {
        let content = fs::read_to_string(Path::new(file_path))
            .expect("Failed to read level file");

        let mut lines = content.lines();
        let mut map_data = Vec::new();
        let mut sprites = Vec::new();
        let mut size = 0;

        while let Some(line) = lines.next() {
            if line.starts_with("[MAP_DATA]") {
                for map_line in lines.by_ref() {
                    if map_line.trim().is_empty() || map_line.starts_with("[") {
                        break;
                    }
                    let row: Vec<u8> = map_line
                        .split(',')
                        .filter_map(|n| n.trim().parse::<u8>().ok())
                        .collect();

                    if size == 0 {
                        size = row.len(); // Assume square map
                    }

                    map_data.extend(row);
                }
            } else if line.starts_with("[SPRITES_DATA]") {
                for sprite_line in lines.by_ref() {
                    if sprite_line.trim().is_empty() || sprite_line.starts_with("[") {
                        break;
                    }
                    
                    let values: Vec<f64> = sprite_line
                        .replace("{", "")
                        .replace("}", "")
                        .split(',')
                        .filter_map(|n| n.trim().parse::<f64>().ok())
                        .collect();

                    if values.len() == 9 {
                        sprites.push(Sprite {
                            x: values[0],
                            y: values[1],
                            vx: values[2],
                            vy: values[3],
                            dir_x: values[4], // Velocity in Y direction
                            dir_y: values[5], // Velocity in Y direction
                            is_projectile: values[6],
                            is_destroyed: values[7],
                            texture: values[8] as i32,
                        });
                    }
                }
            }
        }

        assert_eq!(map_data.len(), size * size, "Map data is not a perfect square!");

        GameMap {
            size,
            map_data,
            sprites,
        }
    }
}
pub fn draw_board(d: &mut RaylibDrawHandle, _player: &Player, _map: &GameMap) {
    let tile_size = 20; 
    let mut y_offset = 0; 

    for row in 0.._map.size {
        let mut x_offset = 0; 

        for col in 0.._map.size {
            let value = _map.map_data[row * _map.size + col];

            let symbol = match value {
                1 => '#',  
                0 => '.',  
                2 => 'O',  
                3 => 'X',  
                4 => '@',  
                _ => ' ',  
            };

            let position_text = format!("{}", symbol);
            d.draw_text(&position_text, x_offset, y_offset, 6, Color::RAYWHITE);

            x_offset += tile_size; 
        }

        y_offset += tile_size; 
    }

    let player_x_offset = _player.pos.x as i32 * tile_size;
    let player_y_offset = _player.pos.y as i32 * tile_size;
    d.draw_text("P", player_x_offset, player_y_offset, 6, Color::RED); 
}
