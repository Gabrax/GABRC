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
    for x in 0..SCREEN_WIDTH {
        let xcam = 2.0 * (x as f32) / SCREEN_WIDTH as f32 - 1.0;

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
            if dir.x < 0.0 { 
                (pos.x - ipos.x) * deltadist.x
            } else { 
                (ipos.x + 1.0 - pos.x) * deltadist.x 
            },
            if dir.y < 0.0 { 
                (pos.y - ipos.y) * deltadist.y
            } else { 
                (ipos.y + 1.0 - pos.y) * deltadist.y 
            },
        );

        let step = Vector2::new(dir.x.signum(), dir.y.signum());

        let mut hit = (0, 0, Vector2::new(0.0, 0.0));

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

            if map_x < 0 || map_x >= MAP_SIZE as i32 || map_y < 0 || map_y >= MAP_SIZE as i32 {
                break;
            }

            hit.0 = MAPDATA[map_y as usize * MAP_SIZE + map_x as usize] as i32;
        }

        let _color = Color::BLACK;
        let texture = match hit.0 {
            1 => &textures[0], 
            2 => &textures[1], 
            3 => &textures[2], 
            4 => &textures[2], 
            _ => &textures[0], 
        };

        // Calculate the position on the texture
        let dperp = if hit.1 == 0 { sidedist.x - deltadist.x } else { sidedist.y - deltadist.y };
        let hit_pos = pos + dir * dperp;
        let mut tex_x = 0.0;
        
        if hit.1 == 0 {
            tex_x = hit_pos.y - hit_pos.y.floor();
        } else {
            tex_x = hit_pos.x - hit_pos.x.floor();
        }

        if (hit.1 == 0 && dir.x > 0.0) || (hit.1 == 1 && dir.y < 0.0) {
            tex_x = 1.0 - tex_x;
        }

        let tex_x = (tex_x * texture.width() as f32) as i32;

        // Calculate the height of the wall slice
        let h = (SCREEN_HEIGHT as f32 / dperp) as i32;
        let y0 = ((SCREEN_HEIGHT / 2) - (h / 2)).max(0);
        let y1 = ((SCREEN_HEIGHT / 2) + (h / 2)).min(SCREEN_HEIGHT - 1);

        // Draw the wall slice
        d.draw_texture_pro(
            texture,
            Rectangle::new(tex_x as f32, 0.0, 1.0, texture.height() as f32),
            Rectangle::new(x as f32, y0 as f32, 1.0, (y1 - y0) as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
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

    let texture_files = [
        "res/greystone.png",
        "res/wood.png",
        "res/mossy.png",
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
    }
}
