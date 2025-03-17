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
    1, 0, 3, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Copy, Clone)]
struct State {
    pos: Vector2,
    dir: Vector2,
    plane: Vector2,
}

impl State {
    fn new() -> Self {
        let dir = {
            let v = Vector2::new(-1.0, 0.1);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            Vector2::new(v.x / len, v.y / len)
        };
        
        State {
            pos: Vector2::new(2.0, 2.0),
            dir,
            plane: Vector2::new(0.0, 0.66),
        }
    }

    fn rotate(&mut self, rot: f32) {
        let cos_rot = rot.cos();
        let sin_rot = rot.sin();
        
        let new_dir_x = self.dir.x * cos_rot - self.dir.y * sin_rot;
        let new_dir_y = self.dir.x * sin_rot + self.dir.y * cos_rot;
        
        let new_plane_x = self.plane.x * cos_rot - self.plane.y * sin_rot;
        let new_plane_y = self.plane.x * sin_rot + self.plane.y * cos_rot;

        self.dir.x = new_dir_x;
        self.dir.y = new_dir_y;
        self.plane.x = new_plane_x;
        self.plane.y = new_plane_y;
    }
}

fn verline(d: &mut RaylibDrawHandle, x: i32, y0: i32, y1: i32, color: Color) {
    for y in y0..=y1 {
        d.draw_pixel(x, y, color);
    }
}

fn render(d: &mut RaylibDrawHandle, state: &State) {
    for x in 0..SCREEN_WIDTH {
        let xcam = 2.0 * (x as f32) / SCREEN_WIDTH as f32 - 1.0;

        let dir = Vector2::new(
            state.dir.x + state.plane.x * xcam,
            state.dir.y + state.plane.y * xcam,
        );

        let pos = state.pos;
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

        let mut color = Color::BLACK;

        match hit.0 {
            1 => color = Color::new(255, 0, 0, 255),
            2 => color = Color::new(0, 255, 0, 255),
            3 => color = Color::new(255, 0, 0, 255),
            4 => color = Color::new(255, 0, 255, 255),
            _ => {}
        }

        if hit.1 == 1 {
            color.r = (color.r as f32 * 0.8) as u8;
            color.g = (color.g as f32 * 0.8) as u8;
        }

        let dperp = if hit.1 == 0 { sidedist.x - deltadist.x } else { sidedist.y - deltadist.y };

        let h = (SCREEN_HEIGHT as f32 / dperp) as i32;
        let y0 = ((SCREEN_HEIGHT / 2) - (h / 2)).max(0);
        let y1 = ((SCREEN_HEIGHT / 2) + (h / 2)).min(SCREEN_HEIGHT - 1);

        verline(d, x, 0, y0, Color::new(32, 32, 32, 255));
        verline(d, x, y0, y1, color);
        verline(d, x, y1, SCREEN_HEIGHT - 1, Color::new(80, 80, 80, 255));
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting with Rust")
        .build();
    rl.set_target_fps(60);

    let mut state = State::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let rotspeed = 3.0 * 0.016;
        let movespeed = 3.0 * 0.016;
        
        if d.is_key_down(KeyboardKey::KEY_LEFT) {
            state.rotate(rotspeed);
        }

        if d.is_key_down(KeyboardKey::KEY_RIGHT) {
            state.rotate(-rotspeed);
        }

        if d.is_key_down(KeyboardKey::KEY_UP) {
            state.pos.x += state.dir.x * movespeed;
            state.pos.y += state.dir.y * movespeed;
        }

        if d.is_key_down(KeyboardKey::KEY_DOWN) {
            state.pos.x -= state.dir.x * movespeed;
            state.pos.y -= state.dir.y * movespeed;
        }

        d.clear_background(Color::WHITE);
        render(&mut d, &state);
    }
}
