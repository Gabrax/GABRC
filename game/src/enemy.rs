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
}

impl Enemy {
    pub fn new(_map: Rc<RefCell<GameMap>>) -> Self {
        let dir = {
            let v = Vector2::new(-1.0, 0.1);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            Vector2::new(v.x / len, v.y / len)
        };
        
        let enemy = Enemy {
            pos: Vector2::new(8.0, 2.0),
            dir,
            projection: Vector2::new(0.0, 0.66),
            movespeed: 3.0 * 0.016,
            _map: _map.clone(),
        };

        let mut map = _map.borrow_mut();

        let enemy_sprite = Sprite {
            x: enemy.pos.x as f64,
            y: enemy.pos.y as f64,
            vx: 0.0,
            vy: 0.0,
            dir_x: enemy.dir.x as f64,
            dir_y: enemy.dir.y as f64,
            is_projectile: 0.0,
            is_destroyed: 0.0,
            texture: 11,
        };

        map.sprites.push(enemy_sprite);

        enemy
    }
}
