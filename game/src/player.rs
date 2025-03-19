
use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub projection: Vector2,
    movespeed: f32,
}

impl Player {
    pub fn new() -> Self {
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
        };
        
        player.rotate(180.0);
        player
    }
    pub fn update(&mut self,_rl: &mut RaylibDrawHandle) {

        self.rotate(-_rl.get_mouse_delta().x * 0.003);

        if _rl.is_key_down(KeyboardKey::KEY_W) {
            self.pos.x += self.dir.x * self.movespeed;
            self.pos.y += self.dir.y * self.movespeed;
        }
        if _rl.is_key_down(KeyboardKey::KEY_S) {
            self.pos.x -= self.dir.x * self.movespeed;
            self.pos.y -= self.dir.y * self.movespeed;
        }
        if _rl.is_key_down(KeyboardKey::KEY_A) { 
            self.pos.x -= self.dir.y * self.movespeed;
            self.pos.y += self.dir.x * self.movespeed;
        }
        if _rl.is_key_down(KeyboardKey::KEY_D) { 
            self.pos.x += self.dir.y * self.movespeed;
            self.pos.y -= self.dir.x * self.movespeed;
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

