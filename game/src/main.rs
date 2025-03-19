use ::core::f32;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const MAP_SIZE: usize = 8;
static MAPDATA: [u8; MAP_SIZE * MAP_SIZE] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 3, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 2, 0, 4, 4, 0, 1,
    1, 0, 0, 0, 4, 0, 0, 1,
    1, 0, 2, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Copy, Clone)]
struct Player {
    pos: Vector2,
    dir: Vector2,
    projection: Vector2,
}

impl Player {
    fn new() -> Self {
        let dir = {
            let v = Vector2::new(-1.0, 0.1);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            Vector2::new(v.x / len, v.y / len)
        };
        
        Player {
            pos: Vector2::new(2.0, 2.0),
            dir,
            projection: Vector2::new(0.0, 0.66),
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

fn verline(d: &mut RaylibDrawHandle, x: i32, y0: i32, y1: i32, color: Color) {
    for y in y0..=y1 {
        d.draw_pixel(x, y, color);
    }
}


fn render(d: &mut RaylibDrawHandle, player: &Player, textures: &[Texture2D]) {
    let floor_texture = &textures[1]; 
    let ceiling_texture = &textures[5]; 
    
    // Floor/Ceiling rendering
    for y in 0..SCREEN_HEIGHT {
        let is_floor = y > SCREEN_HEIGHT / 2; // Floor or ceiling?

        // Use positive camera height for floor, negative for ceiling
        let camera_z = if is_floor {
            0.5 * SCREEN_HEIGHT as f32  // Floor
        } else {
            -0.5 * SCREEN_HEIGHT as f32 // Ceiling (flipped perspective)
        };

        let p = y as f32 - SCREEN_HEIGHT as f32 / 2.0;
        if p == 0.0 {
            continue; // Avoid division by zero
        }

        let row_distance = camera_z / p; // Different for ceiling

        let ray_dir_x0 = player.dir.x - player.projection.x;
        let ray_dir_y0 = player.dir.y - player.projection.y;
        let ray_dir_x1 = player.dir.x + player.projection.x;
        let ray_dir_y1 = player.dir.y + player.projection.y;

        let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / SCREEN_WIDTH as f32;
        let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / SCREEN_WIDTH as f32;

        let mut floor_x = player.pos.x + row_distance * ray_dir_x0;
        let mut floor_y = player.pos.y + row_distance * ray_dir_y0;

        let texture = if is_floor { floor_texture } else { ceiling_texture };

        for x in 0..SCREEN_WIDTH {
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

    // Wall rendering 
    for x in 0..SCREEN_WIDTH {
        // Calculate ray direction
        let xcam = 2.0 * (x as f32) / SCREEN_WIDTH as f32 - 1.0;
        let dir = Vector2::new(
            player.dir.x + player.projection.x * xcam,
            player.dir.y + player.projection.y * xcam,
        );

        let pos = player.pos;
        let mut ipos = Vector2::new(pos.x.floor(), pos.y.floor());

        // Delta distance (avoid division by zero)
        let deltadist = Vector2::new(
            if dir.x.abs() < 1e-20 { 1e30 } else { 1.0 / dir.x.abs() },
            if dir.y.abs() < 1e-20 { 1e30 } else { 1.0 / dir.y.abs() },
        );

        // Side distances
        let mut sidedist = Vector2::new(
            if dir.x < 0.0 { (pos.x - ipos.x) * deltadist.x } else { (ipos.x + 1.0 - pos.x) * deltadist.x },
            if dir.y < 0.0 { (pos.y - ipos.y) * deltadist.y } else { (ipos.y + 1.0 - pos.y) * deltadist.y },
        );

        let step = Vector2::new(dir.x.signum(), dir.y.signum());
        let mut hit = (0, 0, Vector2::new(0.0, 0.0));

        // DDA (Raycasting Loop)
        while hit.0 == 0 {
            if sidedist.x < sidedist.y {
                sidedist.x += deltadist.x;
                ipos.x += step.x;
                hit.1 = 0; // Hit vertical wall
            } else {
                sidedist.y += deltadist.y;
                ipos.y += step.y;
                hit.1 = 1; // Hit horizontal wall
            }

            let map_x = ipos.x as i32;
            let map_y = ipos.y as i32;

            if map_x < 0 || map_x >= MAP_SIZE as i32 || map_y < 0 || map_y >= MAP_SIZE as i32 {
                break;
            }

            hit.0 = MAPDATA[map_y as usize * MAP_SIZE + map_x as usize] as i32;
        }

        // Select texture based on wall type
        let texture = match hit.0 {
            1 => &textures[0], 
            2 => &textures[4], 
            3 => &textures[7], 
            4 => &textures[2], 
            _ => &textures[0], 
        };

        // Calculate perpendicular distance (fix fisheye effect)
        let dperp = if hit.1 == 0 { sidedist.x - deltadist.x } else { sidedist.y - deltadist.y };

        // Hit position in world
        let hit_pos = pos + dir * dperp;
        
        // Corrected texture coordinate
        let mut tex_x = if hit.1 == 0 {
            hit_pos.y - hit_pos.y.floor()
        } else {
            hit_pos.x - hit_pos.x.floor()
        };

        // Flip texture for correct orientation
        if (hit.1 == 0 && dir.x > 0.0) || (hit.1 == 1 && dir.y < 0.0) {
            tex_x = 1.0 - tex_x;
        }

        let tex_x = (tex_x * texture.width() as f32) as i32;

        // Calculate wall height on screen
        let h = (SCREEN_HEIGHT as f32 / dperp) as i32;
        let y0 = ((SCREEN_HEIGHT / 2) - (h / 2)).max(0);
        let y1 = ((SCREEN_HEIGHT / 2) + (h / 2)).min(SCREEN_HEIGHT - 1);

        // Texture scaling fix (prevents stretching)
        let step = texture.height() as f32 / h as f32;
        let mut tex_pos = (y0 as f32 - SCREEN_HEIGHT as f32 / 2.0 + h as f32 / 2.0) * step;

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


fn draw_board(d: &mut RaylibDrawHandle, _player: &Player) {
    let tile_size = 20; // You can adjust this size as per your needs
    let mut y_offset = 0; // This will keep track of the vertical position to draw each row

    for row in 0..MAP_SIZE {
        let mut x_offset = 0; // Reset the horizontal position for each row

        for col in 0..MAP_SIZE {
            let value = MAPDATA[row * MAP_SIZE + col];

            // Determine the symbol for this tile
            let symbol = match value {
                1 => '#',  // Wall
                0 => '.',  // Empty space
                2 => 'O',  // Special object
                3 => 'X',  // Another special object
                4 => '@',  // Another type of object
                _ => ' ',  // Default empty character
            };

            let position_text = format!("{}", symbol);
            // Draw each symbol at the calculated position
            d.draw_text(&position_text, x_offset, y_offset, 6, Color::RAYWHITE);

            x_offset += tile_size; // Move horizontally for the next symbol
        }

        y_offset += tile_size; // Move vertically for the next row
    }

    // Now, draw the player on top of the board
    let player_x_offset = _player.pos.x as i32 * tile_size;
    let player_y_offset = _player.pos.y as i32 * tile_size;
    d.draw_text("P", player_x_offset, player_y_offset, 6, Color::RED);  // Draw 'P' for the player
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting with Rust")
        .vsync()
        .build();

    rl.disable_cursor();

    let mut player = Player::new();
    player.rotate(180.0);

    let texture_files = [
        "res/greystone.png",
        "res/wood.png",
        "res/mossy.png",
        "res/purplestone.png",
        "res/redbrick.png",
        "res/colorstone.png",
        "res/bluestone.png",
        "res/eagle.png",
        "res/barrel.png",
        "res/pillar.png",
        "res/greenlight.png"
    ];

    // Load textures into a vector
    let _textures: Vec<Texture2D> = texture_files
        .iter()
        .map(|&path| rl.load_texture(&thread, path).expect("Failed to load texture"))
        .collect();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let mouse_delta = d.get_mouse_delta();
        let mouse_sensitivity = 0.003;
        player.rotate(-mouse_delta.x * mouse_sensitivity); 

        let movespeed = 3.0 * 0.016;
        if d.is_key_down(KeyboardKey::KEY_W) {
            player.pos.x += player.dir.x * movespeed;
            player.pos.y += player.dir.y * movespeed;
        }
        if d.is_key_down(KeyboardKey::KEY_S) {
            player.pos.x -= player.dir.x * movespeed;
            player.pos.y -= player.dir.y * movespeed;
        }
        if d.is_key_down(KeyboardKey::KEY_A) { 
            player.pos.x -= player.dir.y * movespeed;
            player.pos.y += player.dir.x * movespeed;
        }
        if d.is_key_down(KeyboardKey::KEY_D) { 
            player.pos.x += player.dir.y * movespeed;
            player.pos.y -= player.dir.x * movespeed;
        }

        d.clear_background(Color::WHITE);
        render(&mut d, &player,&_textures);
        draw_board(&mut d, &player);
        d.draw_fps(15, 0);
    }
}
