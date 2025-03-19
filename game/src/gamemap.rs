use std::fs;
use std::path::Path;

use raylib::prelude::*;

use crate::Player;     

#[derive(Clone)]
pub struct GameMap {
    pub size: usize,
    pub map_data: Vec<u8>,
}

impl GameMap {
    pub fn load_from_file(file_path: &str) -> Self {
        let content = fs::read_to_string(Path::new(file_path))
            .expect("Failed to read level file");

        let lines: Vec<&str> = content.lines().collect();
        let size = lines.len(); 

        let map_data: Vec<u8> = lines
            .iter()
            .flat_map(|line| {
                line.split(',')
                    .filter_map(|n| n.trim().parse::<u8>().ok()) 
            })
            .collect();

        assert_eq!(map_data.len(), size * size, "Map data is not a perfect square!");

        GameMap { size, map_data }
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

