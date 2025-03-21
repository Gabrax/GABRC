use raylib::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::GameMap;

#[derive(Clone)]
pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub projection: Vector2,
    movespeed: f32,
    pub _map: Rc<RefCell<GameMap>>,
}

impl Player {
    pub fn new(_map: Rc<RefCell<GameMap>>) -> Self {
        let dir = {
            let v = Vector2::new(-1.0, 0.1);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            Vector2::new(v.x / len, v.y / len)
        };
        
        let mut player = Player {
            pos: Vector2::new(2.0, 2.0),
            dir,
            projection: Vector2::new(0.0, 0.66),
            movespeed: 3.0 * 0.016,
            _map,
        };
        
        player.rotate(180.0);
        player
    }

    pub fn update(&mut self,_rl: &mut RaylibDrawHandle) {

        self.rotate(-_rl.get_mouse_delta().x * 0.003);

        if _rl.is_key_down(KeyboardKey::KEY_W) {
            let new_pos = Vector2::new(self.pos.x + self.dir.x * self.movespeed, self.pos.y + self.dir.y * self.movespeed);
            if self.is_valid_move(new_pos) {
                self.pos = new_pos;
            }
        }
        if _rl.is_key_down(KeyboardKey::KEY_S) {
            let new_pos = Vector2::new(self.pos.x - self.dir.x * self.movespeed, self.pos.y - self.dir.y * self.movespeed);
            if self.is_valid_move(new_pos) {
                self.pos = new_pos;
            }
        }
        if _rl.is_key_down(KeyboardKey::KEY_A) {
            let new_pos = Vector2::new(self.pos.x - self.dir.y * self.movespeed, self.pos.y + self.dir.x * self.movespeed);
            if self.is_valid_move(new_pos) {
                self.pos = new_pos;
            }
        }
        if _rl.is_key_down(KeyboardKey::KEY_D) {
            let new_pos = Vector2::new(self.pos.x + self.dir.y * self.movespeed, self.pos.y - self.dir.x * self.movespeed);
            if self.is_valid_move(new_pos) {
                self.pos = new_pos;
            }
        }
    }

    fn is_valid_move(&self, new_pos: Vector2) -> bool {
        let map = self._map.borrow();
        // Check if the new position is within bounds
        let map_x = new_pos.x.floor() as i32;
        let map_y = new_pos.y.floor() as i32;

        // Check if the new position is within the bounds of the map and on a '0' field
        if map_x >= 0 && map_x < map.size as i32 && map_y >= 0 && map_y < map.size as i32 {
            let map_value = map.map_data[map_y as usize * map.size + map_x as usize];
            map_value == 0 // Only allow movement on '0' fields
        } else {
            false // If out of bounds, don't allow movement
        }
    }

    fn rotate(&mut self, rot: f32) {
        let cos_rot = rot.cos();
        let sin_rot = rot.sin();
        
        let new_dir_x = self.dir.x * cos_rot - self.dir.y * sin_rot;
        let new_dir_y = self.dir.x * sin_rot + self.dir.y * cos_rot;
        
        let new_plane_x = self.projection.x * cos_rot - self.projection.y * sin_rot;
        let new_plane_y = self.projection.x * sin_rot + self.projection.y * cos_rot;

        self.dir.x = new_dir_x;
        self.dir.y = new_dir_y;
        self.projection.x = new_plane_x;
        self.projection.y = new_plane_y;
    }
}

