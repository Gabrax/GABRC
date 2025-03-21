use std::rc::Rc;
use std::cell::RefCell;
use raylib::prelude::*;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::Player;
use crate::GameMap;

pub struct Raycaster {
    pub buffer_width: i32,
    pub buffer_height: i32,
    pub player: Rc<RefCell<Player>>,   
    pub textures: Vec<Rc<RefCell<Image>>>,  
    pub _map: Rc<RefCell<GameMap>>,    
    pixelbuffer: Vec<u32>,
    _framebuffer: RenderTexture2D,
}

impl Raycaster {
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        _framebuffer: RenderTexture2D,
        player: Rc<RefCell<Player>>,
        textures: Vec<Rc<RefCell<Image>>>,  // Use Rc<RefCell<Image>> for mutable access
        _map: Rc<RefCell<GameMap>>,
    ) -> Self {
        let pixelbuffer = vec![0; (screen_width * screen_height) as usize];

        Raycaster {
            buffer_width: screen_width,
            buffer_height: screen_height,
            pixelbuffer,
            player,
            textures,
            _map,
            _framebuffer,
        }
    }

    pub fn render_all(&mut self, d: &mut RaylibDrawHandle) {
        self.render_floor_ceiling();
        self.render_walls();

        self._framebuffer
            .texture_mut()
            .update_texture(bytemuck::cast_slice(&self.pixelbuffer));
        
        d.draw_texture_pro(
            self._framebuffer.texture(), // Source texture
            rrect(0, 0, self.buffer_width as f32, -self.buffer_height as f32), // Source rectangle (flipped vertically)
            rrect(0, 0, d.get_screen_width() as f32, d.get_screen_height() as f32), // Destination rectangle (stretched to window)
            rvec2(0.0, 0.0), // Origin
            0.0, // Rotation (0 means no rotation)
            Color::WHITE, // Tint (WHITE means no color modification)
        );
    }

    pub fn color_to_u32(color: Color) -> u32 {
        // Swap red and blue channels for BGRA format
        ((color.a as u32) << 24) | ((color.b as u32) << 16) | ((color.g as u32) << 8) | (color.r as u32)
    }

    pub fn render_floor_ceiling(&mut self) {
        let mut floor_texture = self.textures[3].borrow_mut();  // Borrow the image immutably
        let mut ceiling_texture = self.textures[6].borrow_mut();  // Borrow the image immutably

        let player = self.player.borrow();

        let ray_dir_x0 = player.dir.x - player.projection.x;
        let ray_dir_y0 = player.dir.y - player.projection.y;
        let ray_dir_x1 = player.dir.x + player.projection.x;
        let ray_dir_y1 = player.dir.y + player.projection.y;

        let pos_z = 0.5 * self.buffer_height as f32;

        for y in (self.buffer_height / 2 + 1)..self.buffer_height {
            let p = y as f32 - self.buffer_height as f32 / 2.0;
            let row_distance = pos_z / p;

            let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / self.buffer_width as f32;
            let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / self.buffer_width as f32;

            let mut floor_x = player.pos.x + row_distance * ray_dir_x0;
            let mut floor_y = player.pos.y + row_distance * ray_dir_y0;

            for x in 0..self.buffer_width {
                let cell_x = floor_x as i32;
                let cell_y = floor_y as i32;

                let tex_x = ((floor_x - cell_x as f32) * floor_texture.width as f32) as i32 & (floor_texture.width - 1);
                let tex_y = ((floor_y - cell_y as f32) * floor_texture.height as f32) as i32 & (floor_texture.height - 1);

                floor_x += floor_step_x;
                floor_y += floor_step_y;

                let floor_color = floor_texture.get_color(tex_x, tex_y);
                // Store the floor pixel color in pixelbuffer
                self.pixelbuffer[(y * self.buffer_width + x) as usize] = Self::color_to_u32(floor_color);

                let ceiling_color = ceiling_texture.get_color(tex_x, tex_y);
                // Store the ceiling pixel color in pixelbuffer (mirrored y-coordinate)
                self.pixelbuffer[((self.buffer_height - y - 1) * self.buffer_width + x) as usize] = Self::color_to_u32(ceiling_color);
            }
        }
    }


    pub fn render_walls(&mut self) {
        let player = self.player.borrow();
        let _map = self._map.borrow();

        for x in 0..self.buffer_width {
            let xcam = 2.0 * (x as f32) / self.buffer_width as f32 - 1.0;
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

            // Select the correct texture based on wall type
            let texture_index = match hit.0 {
                1 => 0,  // Texture for wall type 1
                2 => 4,  // Texture for wall type 2
                3 => 11, // Texture for wall type 3
                4 => 2,  // Texture for wall type 4
                _ => 0,  // Default texture
            };

            let mut texture = self.textures[texture_index].borrow_mut();
            let tex_width = texture.width();
            let tex_height = texture.height();

            // Compute the perpendicular distance to the wall
            let dperp = if hit.1 == 0 { sidedist.x - deltadist.x } else { sidedist.y - deltadist.y };
            let h = (self.buffer_height as f32 / dperp) as i32;
            let y0 = ((self.buffer_height / 2) - (h / 2)).max(0);
            let y1 = ((self.buffer_height / 2) + (h / 2)).min(self.buffer_height - 1);

            // Compute texture X coordinate
            let hit_pos = pos + dir * dperp;
            let mut tex_x = if hit.1 == 0 {
                hit_pos.y - hit_pos.y.floor()
            } else {
                hit_pos.x - hit_pos.x.floor()
            };

            // Flip texture coordinate based on ray direction
            if (hit.1 == 0 && dir.x > 0.0) || (hit.1 == 1 && dir.y < 0.0) {
                tex_x = 1.0 - tex_x;
            }

            let tex_x = (tex_x * tex_width as f32) as i32;
            let step = tex_height as f32 / h as f32;
            let mut tex_pos = (y0 as f32 - self.buffer_height as f32 / 2.0 + h as f32 / 2.0) * step;

            for y in y0..y1 {
                let tex_y = (tex_height - 1 - (tex_pos as i32)) & (tex_height - 1);
                tex_pos += step;

                let color = texture.get_color(tex_x, tex_y);

                // Store the pixel in the buffer
                self.pixelbuffer[(y * self.buffer_width + x) as usize] = Self::color_to_u32(color);
            }
        }
    }
}
