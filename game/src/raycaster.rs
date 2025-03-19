

use std::rc::Rc;
use std::cell::RefCell;
use raylib::prelude::*;

use crate::Player;
use crate::GameMap;

#[derive(Clone)]
pub struct Raycaster {
    pub screen_width: i32,
    pub screen_height: i32,
    pub pixelbuffer: Vec<Vec<u32>>,
    pub player: Rc<RefCell<Player>>,   // Allow mutation
    pub textures: Vec<Rc<Texture2D>>,  // Use Rc<Texture2D> for shared ownership
    pub _map: Rc<RefCell<GameMap>>,    // Shared ownership of game map
}

impl Raycaster {
    // Constructor
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        player: Rc<RefCell<Player>>,
        textures: Vec<Rc<Texture2D>>,
        _map: Rc<RefCell<GameMap>>,
    ) -> Self {
        let pixelbuffer = vec![vec![0; screen_width as usize]; screen_height as usize];

        Raycaster {
            screen_width,
            screen_height,
            pixelbuffer,
            player,
            textures,
            _map,
        }
    }
    pub fn render_all(&self, d: &mut RaylibDrawHandle) {
        self.render_floor_ceiling(d);
        self.render_walls(d);
    }

    pub fn render_floor_ceiling(&self, d: &mut RaylibDrawHandle) {
        let floor_texture = &*self.textures[1];
        let ceiling_texture = &*self.textures[5];

        let player = self.player.borrow();

        for y in 0..self.screen_height {
            let is_floor = y > self.screen_height / 2;
            let camera_z = if is_floor {
                0.5 * self.screen_height as f32
            } else {
                -0.5 * self.screen_height as f32
            };

            let p = y as f32 - self.screen_height as f32 / 2.0;
            if p == 0.0 {
                continue;
            }

            let row_distance = camera_z / p;

            let ray_dir_x0 = player.dir.x - player.projection.x;
            let ray_dir_y0 = player.dir.y - player.projection.y;
            let ray_dir_x1 = player.dir.x + player.projection.x;
            let ray_dir_y1 = player.dir.y + player.projection.y;

            let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / self.screen_width as f32;
            let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / self.screen_width as f32;

            let mut floor_x = player.pos.x + row_distance * ray_dir_x0;
            let mut floor_y = player.pos.y + row_distance * ray_dir_y0;

            let texture = if is_floor { floor_texture } else { ceiling_texture };

            for x in 0..self.screen_width {
                let cell_x = floor_x as i32;
                let cell_y = floor_y as i32;

                let tex_x = ((floor_x - cell_x as f32) * texture.width() as f32) as i32;
                let tex_y = ((floor_y - cell_y as f32) * texture.height() as f32) as i32;

                floor_x += floor_step_x;
                floor_y += floor_step_y;

                d.draw_texture_pro(
                    texture,
                    Rectangle::new(tex_x as f32, tex_y as f32, 1.0, 1.0),
                    Rectangle::new(x as f32, y as f32, 1.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }

    pub fn render_walls(&self, d: &mut RaylibDrawHandle) {
        let player = self.player.borrow();
        let _map = self._map.borrow();

        for x in 0..self.screen_width {
            let xcam = 2.0 * (x as f32) / self.screen_width as f32 - 1.0;
            let dir = Vector2::new(
                player.dir.x + player.projection.x * xcam,
                player.dir.y + player.projection.y * xcam,
            );

            let pos = player.pos;
            let mut ipos = Vector2::new(pos.x.floor(), pos.y.floor());

            let deltadist = Vector2::new(
                if dir.x.abs() < 1e-20 { 1e30 } else { 1.0 / dir.x.abs() },
                if dir.y.abs() < 1e-20 { 1e30 } else { 1.0 / dir.y.abs() },
            );

            let mut sidedist = Vector2::new(
                if dir.x < 0.0 { (pos.x - ipos.x) * deltadist.x } else { (ipos.x + 1.0 - pos.x) * deltadist.x },
                if dir.y < 0.0 { (pos.y - ipos.y) * deltadist.y } else { (ipos.y + 1.0 - pos.y) * deltadist.y },
            );

            let step = Vector2::new(dir.x.signum(), dir.y.signum());
            let mut hit = (0, 0);

            while hit.0 == 0 {
                if sidedist.x < sidedist.y {
                    sidedist.x += deltadist.x;
                    ipos.x += step.x;
                    hit.1 = 0;
                } else {
                    sidedist.y += deltadist.y;
                    ipos.y += step.y;
                    hit.1 = 1;
                }

                let map_x = ipos.x as i32;
                let map_y = ipos.y as i32;

                if map_x < 0 || map_x >= _map.size as i32 || map_y < 0 || map_y >= _map.size as i32 {
                    break;
                }

                hit.0 = _map.map_data[map_y as usize * _map.size + map_x as usize] as i32;
            }

            let texture = match hit.0 {
                1 => &*self.textures[0], 
                2 => &*self.textures[4], 
                3 => &*self.textures[11], 
                4 => &*self.textures[2], 
                _ => &*self.textures[0], 
            };

            let dperp = if hit.1 == 0 { sidedist.x - deltadist.x } else { sidedist.y - deltadist.y };
            let hit_pos = pos + dir * dperp;

            let mut tex_x = if hit.1 == 0 {
                hit_pos.y - hit_pos.y.floor()
            } else {
                hit_pos.x - hit_pos.x.floor()
            };

            if (hit.1 == 0 && dir.x > 0.0) || (hit.1 == 1 && dir.y < 0.0) {
                tex_x = 1.0 - tex_x;
            }

            let tex_x = (tex_x * texture.width() as f32) as i32;
            let h = (self.screen_height as f32 / dperp) as i32;
            let y0 = ((self.screen_height / 2) - (h / 2)).max(0);
            let y1 = ((self.screen_height / 2) + (h / 2)).min(self.screen_height - 1);

            let step = texture.height() as f32 / h as f32;
            let mut tex_pos = (y0 as f32 - self.screen_height as f32 / 2.0 + h as f32 / 2.0) * step;

            for y in y0..y1 {
                let tex_y = (tex_pos as i32) & (texture.height() - 1);
                tex_pos += step;

                d.draw_texture_pro(
                    texture,
                    Rectangle::new(tex_x as f32, tex_y as f32, 1.0, 1.0),
                    Rectangle::new(x as f32, y as f32, 1.0, 1.0),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }
}
