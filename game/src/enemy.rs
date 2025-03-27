use raylib::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::GameMap;
use crate::gamemap::Sprite;

#[derive(Clone)]
pub struct Enemy {
    pub pos: Vector2,
    pub dir: Vector2,
    pub projection: Vector2,
    movespeed: f32,
    _map: Rc<RefCell<GameMap>>,
    sprite_index: usize,
    frame_counter: usize, 
}

impl Enemy {
    pub fn new(_map: Rc<RefCell<GameMap>>) -> Self {
        let dir = {
            let v = Vector2::new(-1.0, 0.1);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            Vector2::new(v.x / len, v.y / len)
        };
        
        let mut map = _map.borrow_mut();

        let enemy_sprite = Sprite {
            x: 8.0,
            y: 2.0,
            vx: 0.0,
            vy: 0.0,
            dir_x: dir.x as f64,
            dir_y: dir.y as f64,
            is_projectile: 0.0,
            is_ui: 0.0,
            is_destroyed: 0.0,
            texture: 13,
        };

        let sprite_index = map.sprites.len();
        map.sprites.push(enemy_sprite);

        Enemy {
            pos: Vector2::new(8.0, 2.0),
            dir,
            projection: Vector2::new(0.0, 0.66),
            movespeed: 3.0 * 0.016,
            _map: _map.clone(),
            sprite_index,
            frame_counter: 0, 
        }
    }

    pub fn update(&mut self) {
        self.frame_counter += 1;

        // Change texture every 10 frames (adjust for desired speed)
        if self.frame_counter % 10 == 0 {
            let mut map = self._map.borrow_mut();
            if let Some(sprite) = map.sprites.get_mut(self.sprite_index) {
                sprite.texture = 13 + ((sprite.texture - 13 + 1) % 4);
            }
        }
    }
}
