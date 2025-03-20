use raylib::ffi::UnloadImage;
use raylib::prelude::*;

use std::fmt;
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

    let _map: Vec<u8> = vec![
        1, 1, 1, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 3, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 2, 0, 4, 4, 0, 1,
        1, 0, 0, 0, 4, 0, 0, 1,
        1, 0, 2, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("GPU Raycast")
        .vsync()
        .build();
    //rl.set_target_fps(60);
    rl.disable_cursor();

    let mut _shader = rl.load_shader(&thread,None,Some("res/base.glsl"),).unwrap();

    let _map_loc = _shader.get_shader_location("mapData");
    let _player_pos_x_loc = _shader.get_shader_location("player_pos_x");
    let _player_pos_y_loc = _shader.get_shader_location("player_pos_y");
    let _player_dir_x_loc = _shader.get_shader_location("player_dir_x");
    let _player_dir_y_loc = _shader.get_shader_location("player_dir_y");
    let _player_proj_x_loc = _shader.get_shader_location("player_proj_x");
    let _player_proj_y_loc = _shader.get_shader_location("player_proj_x");

    let _map_f32: Vec<f32> = _map.iter().map(|&v| v as f32).collect();

    let mut target = rl.load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();

    let mut _player = Player::new();

    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);


        _player.update(&mut d);

        //println!("{}", _player.dir.x);

        _shader.set_shader_value_v(_map_loc, &_map_f32);
        _shader.set_shader_value(_player_pos_x_loc, _player.pos.x);
        _shader.set_shader_value(_player_pos_y_loc, _player.pos.y);
        _shader.set_shader_value(_player_dir_x_loc, _player.dir.x);
        _shader.set_shader_value(_player_dir_y_loc, _player.dir.x);
        _shader.set_shader_value(_player_proj_x_loc, _player.projection.x);
        _shader.set_shader_value(_player_proj_y_loc, _player.projection.x);


        {
            let mut d = d.begin_texture_mode(&thread, &mut target); // Enable drawing to texture

            //d.clear_background(Color::RAYWHITE); 
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
