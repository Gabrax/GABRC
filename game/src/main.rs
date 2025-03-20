use raylib::ffi::UnloadImage;
use raylib::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
mod player;
use player::Player;

mod gamemap;
use gamemap::GameMap;
use crate::gamemap::draw_board;

mod raycaster;
use raycaster::Raycaster;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;


fn main() {

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib [shaders] example - texture drawing")
        .build();

    let mut _shader = rl.load_shader(
                &thread,
                None,
                Some(
                    "res/base.glsl"
                ),
            )
            .unwrap();

    let _time_loc = _shader.get_shader_location("time");

    let mut target = rl
        .load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .unwrap();


    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second


    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);

        _shader.set_shader_value(_time_loc, d.get_time() as f32);
        {
            let mut d = d.begin_texture_mode(&thread, &mut target); // Enable drawing to texture

            d.clear_background(Color::RAYWHITE); // Clear texture background
            {

            }
        }

        {
            let mut d = d.begin_shader_mode(&_shader);

            d.draw_texture_rec(
                target.texture(),
                rrect(0, 0, target.texture.width, -target.texture.height),
                rvec2(0, 0),
                Color::WHITE,
            );
        }

        d.draw_fps(700, 15);
    }

}

//fn main() {
//    let (mut rl, thread) = raylib::init()
//        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
//        .title("Raycasting with Rust")
//        .vsync()
//        .build();
//
//    rl.disable_cursor();
//
//    let texture_files = [
//        "res/greystone.png",
//        "res/wood.png",
//        "res/mossy.png",
//        "res/purplestone.png",
//        "res/redbrick.png",
//        "res/colorstone.png",
//        "res/bluestone.png",
//        "res/eagle.png",
//        "res/barrel.png",
//        "res/pillar.png",
//        "res/greenlight.png",
//        "res/chomik.png",
//    ];
//
//    let textures: Vec<Rc<RefCell<Image>>> = texture_files
//        .iter()
//        .map(|&path| Rc::new(RefCell::new(Image::load_image(path).expect("Failed to load texture"))))
//        .collect();
//
//    let game_map = Rc::new(RefCell::new(GameMap::load_from_file("res/level_1.txt")));
//
//    let player = Rc::new(RefCell::new(Player::new()));
//
//    let raycaster = Raycaster::new(
//        SCREEN_WIDTH,
//        SCREEN_HEIGHT,
//        Rc::clone(&player),
//        textures,
//        Rc::clone(&game_map),
//    );
//
//    while !rl.window_should_close() {
//        let mut d = rl.begin_drawing(&thread);
//
//        raycaster.render_all(&mut d);
//        player.borrow_mut().update(&mut d);
//
//        draw_board(&mut d, &player.borrow(), &game_map.borrow());
//        d.draw_fps(15, 0);
//    }
//}
