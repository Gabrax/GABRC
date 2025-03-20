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
        "res/chomik.png",
    ];

    let textures: Vec<Rc<RefCell<Image>>> = texture_files
        .iter()
        .map(|&path| Rc::new(RefCell::new(Image::load_image(path).expect("Failed to load texture"))))
        .collect();

    let game_map = Rc::new(RefCell::new(GameMap::load_from_file("res/level_1.txt")));

    let player = Rc::new(RefCell::new(Player::new()));

    let raycaster = Raycaster::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
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
