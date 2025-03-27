use raylib::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
mod player;
use player::Player;

mod enemy;
use enemy::Enemy;

mod gamemap;
use gamemap::GameMap;
use crate::gamemap::draw_board;

mod raycaster;
use raycaster::Raycaster;

const BUFFER_WIDTH: i32 = 550;
const BUFFER_HEIGHT: i32 = 350;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Raycasting with Rust")
        .vsync()
        .build();

    rl.disable_cursor();


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
        "res/greenlight.png",
        "res/demon.png",
        "res/bullet.png"
    ];

    let textures: Vec<Rc<RefCell<Image>>> = texture_files
        .iter()
        .map(|&path| Rc::new(RefCell::new(Image::load_image(path).expect("Failed to load texture"))))
        .collect();

    let game_map = Rc::new(RefCell::new(GameMap::load_map("res/level_1.txt")));

    let player = Rc::new(RefCell::new(Player::new(game_map.clone())));
    let enemy = Rc::new(RefCell::new(Enemy::new(game_map.clone())));
    let _framebuffer = rl
    .load_render_texture(&thread, BUFFER_WIDTH as u32, BUFFER_HEIGHT as u32)
    .unwrap();

    let mut raycaster = Raycaster::new(
        BUFFER_WIDTH,
        BUFFER_HEIGHT,
        _framebuffer,
        Rc::clone(&player),
        textures,
        Rc::clone(&game_map),
    );

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        raycaster.render_all(&mut d);
        player.borrow_mut().update(&mut d);

        draw_board(&mut d, &player.borrow(), &game_map.borrow());
        d.draw_fps(15, 0);
    }
}
