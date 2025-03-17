extern crate raylib;
use raylib::prelude::*; 

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;
const MAP_WIDTH: usize = 22;
const MAP_HEIGHT: usize = 24;

const WORLD_MAP: [[i32; MAP_HEIGHT]; MAP_WIDTH] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
];

fn main() {
    let (mut rl, thread) = init().size(WINDOW_WIDTH, WINDOW_HEIGHT).title("Raycasting with Rust").build();
    rl.set_target_fps(60);

    let mut player_x: f32 = 12.0;
    let mut player_y: f32 = 12.0;
    let mut player_angle: f32 = 0.0;

    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);

        // Clear the screen
        d.clear_background(Color::WHITE);

        // Process user input
        if d.is_key_down(KeyboardKey::KEY_A) {
            player_angle -= 0.05; // Rotate left
        }
        if d.is_key_down(KeyboardKey::KEY_D) {
            player_angle += 0.05; // Rotate right
        }
        if d.is_key_down(KeyboardKey::KEY_W) {
            player_x += player_angle.cos() * 0.1; // Move forward
            player_y += player_angle.sin() * 0.1;
        }
        if d.is_key_down(KeyboardKey::KEY_S) {
            player_x -= player_angle.cos() * 0.1; // Move backward
            player_y -= player_angle.sin() * 0.1;
        }

        // Basic raycasting logic
        for ray in 0..WINDOW_WIDTH {
            let angle = player_angle + (ray as f32) * 0.00392; // Slightly change the angle for each ray
            let mut distance = 0.0;

            // Cast the ray
            loop {
                let ray_x = player_x + distance * angle.cos();
                let ray_y = player_y + distance * angle.sin();

                let map_x = ray_x as usize;
                let map_y = ray_y as usize;

                // Check if the ray hits a wall
                if map_x >= MAP_WIDTH || map_y >= MAP_HEIGHT || WORLD_MAP[map_x][map_y] == 1 {
                    break;
                }

                distance += 0.1;
            }

            // Draw the ray
            let height = (WINDOW_HEIGHT as f32) / (distance * 0.1);
            d.draw_line(ray as i32, WINDOW_HEIGHT / 2 - height as i32 / 2, ray as i32, WINDOW_HEIGHT / 2 + height as i32 / 2, Color::BLACK);
        }
    }
}

