use raylib::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::GameMap;
use crate::gamemap::Sprite;

#[derive(Clone)]
pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub projection: Vector2,
    movespeed: f32,
    _map: Rc<RefCell<GameMap>>,
    is_shooting: bool,
    sprite_index: usize,
    frame_counter: usize
}

impl Player {
    pub fn new(_map: Rc<RefCell<GameMap>>) -> Self {
        let dir = {
            let v = Vector2::new(-1.0, 0.1);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            Vector2::new(v.x / len, v.y / len)
        };


        let mut map = _map.borrow_mut();

        let shotgun_sprite = Sprite {
            x: 2.0,
            y: 2.0,
            vx: 0.0,
            vy: 0.0,
            dir_x: dir.x as f64,
            dir_y: dir.y as f64,
            is_projectile: 0.0,
            is_ui: 1.0,
            is_destroyed: 0.0,
            texture: 17,
        };

        let sprite_index = map.sprites.len();
        map.sprites.push(shotgun_sprite);
        
        let mut player = Player {
            pos: Vector2::new(2.0, 2.0),
            dir,
            projection: Vector2::new(0.0, 0.66),
            movespeed: 3.0 * 0.016,
            _map: _map.clone(),
            is_shooting: false,
            sprite_index,
            frame_counter: 0,
        };
        
        player.rotate(180.0);
        player
    }

    pub fn update(&mut self, _rl: &mut RaylibDrawHandle) {
        self.frame_counter += 1;

        let mut spawn_bullet = None; // Store bullet separately

        {
            let mut map = self._map.borrow_mut();
            if let Some(sprite) = map.sprites.get_mut(self.sprite_index) {
                if self.is_shooting && self.frame_counter % 10 == 0 {
                    sprite.texture += 1;

                    if sprite.texture == 19 {
                        spawn_bullet = Some(Sprite {
                            x: self.pos.x as f64,
                            y: self.pos.y as f64,
                            vx: 0.0,
                            vy: 0.0,
                            dir_x: self.dir.x as f64,
                            dir_y: self.dir.y as f64,
                            is_projectile: 1.0,
                            is_ui: 0.0,
                            is_destroyed: 0.0,
                            texture: 12,
                        });
                    }

                    if sprite.texture > 18 + 6 {
                        sprite.texture = 17;
                        self.is_shooting = false;
                    }
                }
            }
        } // `_map` borrow is dropped here

        // Now that `_map` is no longer borrowed, we can safely push the bullet
        if let Some(bullet) = spawn_bullet {
            let mut map = self._map.borrow_mut();
            map.sprites.push(bullet);
        }

        self.rotate(-_rl.get_mouse_delta().x * 0.003);

        if _rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.shoot();
        }

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

    fn shoot(&mut self) {
        if self.is_shooting {
            return; // Prevent shooting again until animation resets
        }

        self.is_shooting = true;
        self.frame_counter = 0; // Reset animation counter

        let mut map = self._map.borrow_mut();
        if let Some(sprite) = map.sprites.get_mut(self.sprite_index) {
            sprite.texture = 17; // Start animation from the first frame
        }
    }

    fn spawn_bullet(&mut self) {
        let mut map = self._map.borrow_mut();

        
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

