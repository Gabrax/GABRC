use std::rc::Rc;
use std::cell::RefCell;
use raylib::prelude::*;

use rand::random;
use crate::Player;
use crate::Enemy;
use crate::GameMap;

pub struct Raycaster
{
    buffer_width: i32,
    buffer_height: i32,
    player: Rc<RefCell<Player>>,   
    textures: Vec<Rc<RefCell<Image>>>,  
    _map: Rc<RefCell<GameMap>>,    
    pixelbuffer: Vec<u32>,
    _framebuffer: RenderTexture2D,
    z_buffer: Vec<f64>,
    sprite_order: Vec<i32>,
    sprite_distance: Vec<f64>
}

impl Raycaster
{
    pub fn new
    (
        screen_width: i32,
        screen_height: i32,
        _framebuffer: RenderTexture2D,
        player: Rc<RefCell<Player>>,
        textures: Vec<Rc<RefCell<Image>>>,  // Use Rc<RefCell<Image>> for mutable access
        _map: Rc<RefCell<GameMap>>
    ) -> Self
    {
        let pixelbuffer = vec![0; (screen_width * screen_height) as usize];
        let z_buffer = vec![0.0; screen_width as usize]; // Stores depth values for each column
        let sprite_order = Vec::new(); // Will store indices of sorted sprites
        let sprite_distance = Vec::new(); // Will store distances of sprites from player

        Raycaster {
            buffer_width: screen_width,
            buffer_height: screen_height,
            pixelbuffer,
            player,
            textures,
            _map,
            _framebuffer,
            z_buffer,
            sprite_order,
            sprite_distance
        }
    }

    pub fn render_all(&mut self, d: &mut RaylibDrawHandle) {
        self.render_floor_ceiling();
        self.render_walls();
        self.render_sprites(d);

        let mut sprites = self._map.borrow_mut();
        let vec = &mut sprites.sprites;
        vec.retain(|sprite| sprite.is_destroyed == 0.0);

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

    fn color_to_u32(color: Color) -> u32 {
        // Swap red and blue channels for BGRA format
        ((color.a as u32) << 24) | ((color.b as u32) << 16) | ((color.g as u32) << 8) | (color.r as u32)
    }

    fn sort_sprites(order: &mut [i32], dist: &mut [f64]) {
        let mut sprites: Vec<(f64, i32)> = order.iter().zip(dist.iter()).map(|(&o, &d)| (d, o)).collect();

        // Sort in descending order based on distance
        sprites.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Restore sorted values back into the arrays
        for (i, (d, o)) in sprites.iter().enumerate() {
            dist[i] = *d;
            order[i] = *o;
        }
    }

    fn render_floor_ceiling(&mut self) {
        let mut floor_texture = self.textures[6].borrow_mut();  // Borrow the image immutably
        let mut ceiling_texture = self.textures[1].borrow_mut();  // Borrow the image immutably

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


    fn render_walls(&mut self) {
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
                3 => 7, // Texture for wall type 3
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

                self.pixelbuffer[(y * self.buffer_width + x) as usize] = Self::color_to_u32(color);
            }

            self.z_buffer[x as usize] = dperp as f64; 
        }
    }

    fn render_sprites(&mut self,d: &mut RaylibDrawHandle) {
        let mut sprites = self._map.borrow_mut();
        let map_size = sprites.size;
        let _map_data = sprites.map_data.clone();
        let vec = &mut sprites.sprites;
        let player = self.player.borrow();
        let pos = player.pos;

        self.sprite_order.resize(vec.len(), 0);
        self.sprite_distance.resize(vec.len(), 0.0);

        for (i, item) in vec.iter().enumerate() {
            self.sprite_order[i] = i as i32;
            self.sprite_distance[i] = (pos.x as f64 - item.x).powi(2) + (pos.y as f64 - item.y).powi(2);
        }

        Self::sort_sprites(&mut self.sprite_order, &mut self.sprite_distance);

        let w = self.buffer_width as f32;
        let h = self.buffer_height as f32;

        let mut projectile_ui_index = 0;

        for i in 0..vec.len() {
            let sprite_index = self.sprite_order[i] as usize;
            let sprite = &mut vec[sprite_index]; // Mutable reference

            let mut texture = self.textures[sprite.texture as usize].borrow_mut();
            let tex_width = texture.width;
            let tex_height = texture.height;

            if sprite.is_projectile == 1.0 {
                let dir_x = sprite.dir_x;
                let dir_y = sprite.dir_y;

                // Normalize direction
                let length = (dir_x * dir_x + dir_y * dir_y).sqrt();
                if length != 0.0 {
                    // Add noise to the direction vector to introduce more deviation
                    let noise_x = (random::<f32>() - 0.5) * 0.2; // Adjust the multiplier for more/less noise
                    let noise_y = (random::<f32>() - 0.5) * 0.2;

                    // Apply the noise to the direction components
                    let vx = (dir_x / length + noise_x as f64) * 0.1;
                    let vy = (dir_y /length + noise_y as f64) * 0.1;

                    let dt = d.get_time().clamp(0.0, 1.0); 

                    sprite.x += vx * dt; 
                    sprite.y += vy * dt;

                    let grid_x = sprite.x as usize; 
                    let grid_y = sprite.y as usize;

                    let index = grid_y * map_size + grid_x; 

                    if index < _map_data.len() && _map_data[index] != 0 {
                        sprite.is_destroyed = 1.0; 
                    }
                }
            }

            //if sprite.is_projectile == 1.0 {
            //    // Define position for the projectile UI on the right side of the screen
            //    let screen_x = (w * 0.85) as i32; // Right side of the screen
            //    // Use i32 for the Y position, casting the index safely
            //    let screen_y = (h * 0.1 + (projectile_ui_index as f32 * (tex_height as f32 + 5.0))) as i32;
            //    projectile_ui_index += 1;
            //
            //    // Iterate over the sprite texture
            //    for y in 0..tex_height {
            //        for x in 0..tex_width {
            //            let color = texture.get_color(x, y);
            //            if color.a > 0 { // Ignore transparent pixels
            //                let pixel_x = screen_x + x;
            //                let pixel_y = screen_y + y;
            //
            //                // Ensure coordinates are within the buffer's bounds
            //                if pixel_x >= 0 && pixel_x < self.buffer_width &&
            //                   pixel_y >= 0 && pixel_y < self.buffer_height {
            //                    // Convert to buffer index and set the pixel color
            //                    let buffer_index = pixel_y as usize * self.buffer_width as usize + pixel_x as usize;
            //                    self.pixelbuffer[buffer_index] = Self::color_to_u32(color);
            //                }
            //            }
            //        }
            //    }
            //    continue; // Skip 3D rendering for this projectile
            //}


            let sprite_x = sprite.x as f32 - pos.x;
            let sprite_y = sprite.y as f32 - pos.y;

            let inv_det = 1.0 / (player.projection.x * player.dir.y - player.dir.x * player.projection.y);
            let transform_x = inv_det * (player.dir.y * sprite_x - player.dir.x * sprite_y);
            let transform_y = inv_det * (-player.projection.y * sprite_x + player.projection.x * sprite_y);

            if transform_y <= 0.0 {
                continue; // Skip sprites behind the player
            }

            let sprite_screen_x = ((w / 2.0) * (1.0 + transform_x / transform_y)).round() as i32;
            let sprite_height = (h / transform_y).abs() as i32;
            let v_move_screen = 0;

            let draw_start_y = (-sprite_height / 2 + self.buffer_height / 2 ).clamp(0, self.buffer_height);
            let draw_end_y = (sprite_height / 2 + self.buffer_height / 2 ).clamp(0, self.buffer_height - 1);

            let sprite_width = (h / transform_y).abs() as i32;
            let draw_start_x = (-sprite_width / 2 + sprite_screen_x).clamp(0, self.buffer_width);
            let draw_end_x = (sprite_width / 2 + sprite_screen_x).clamp(0, self.buffer_width);

            for stripe in draw_start_x..draw_end_x {
                let tex_x = (((stripe - (-sprite_width / 2 + sprite_screen_x)) * tex_width) / sprite_width).clamp(0, tex_width - 1);

                if transform_y > 0.0 && (stripe as usize) < self.z_buffer.len() && (transform_y as f64) < self.z_buffer[stripe as usize] {
                    for y in draw_start_y..draw_end_y {
                        let d = (y - v_move_screen) * 256 - (self.buffer_height * 128) + (sprite_height * 128);
                        let tex_y = tex_height - 1 - ((d * tex_height) / sprite_height / 256).clamp(0, tex_height - 1);

                        let color = texture.get_color(tex_x, tex_y);
                        if color.a > 0 && !(color.r == 0 && color.g == 0 && color.b == 0) {
                            let buffer_index = (y * self.buffer_width + stripe) as usize;
                            self.pixelbuffer[buffer_index] = Self::color_to_u32(color);
                        }
                    }
                }
            }
        }
    }
}
