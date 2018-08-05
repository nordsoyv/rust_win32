#![feature(use_extern_macros)]
extern crate game_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate wasm_bindgen;

use game_core::game_init;
use game_core::game_loop;
use game_core::GameInput;
use wasm_bindgen::prelude::*;
use game_core::entities::Color;

#[wasm_bindgen(module = "./platform")]
extern {
    fn random() -> f32;
    fn log(s: String);
    fn start_frame();
    fn end_frame();
    fn draw_rectangle(min_x: f32, min_y: f32, max_x: f32, max_y: f32, red: f32, green: f32, blue: f32);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInput {
    pub up_key: bool,
    pub down_key: bool,
    pub left_key: bool,
    pub right_key: bool,
    pub shoot_right: bool,
    pub shoot_left: bool,
    pub shoot_up: bool,
    pub shoot_down: bool,
    pub quit_key: bool,
    pub space: bool,
}


impl PlayerInput {
    fn to_game_input(&self) -> GameInput {
        GameInput {
            up_key: self.up_key,
            down_key: self.down_key,
            left_key: self.left_key,
            right_key: self.right_key,
            shoot_right: self.shoot_right,
            shoot_left: self.shoot_left,
            shoot_up: self.shoot_up,
            shoot_down: self.shoot_down,
            quit_key: self.quit_key,
            space: self.space,
        }
    }
}

fn draw_rectangle_inner (min_x: f32, min_y: f32, max_x: f32, max_y: f32, color:Color) {
    draw_rectangle(min_x,min_y,max_x,max_y,color.r,color.g,color.b);
}

#[wasm_bindgen]
pub fn init() {
    let platform = game_core::Platform {
        random,
        log,
        start_frame,
        end_frame,
        draw_rectangle: draw_rectangle_inner
    };
    game_init(960.0, 540.0, platform);
}

#[wasm_bindgen]
pub fn update(input_string: String, time_elapsed: f32, delta: f32)  {
    let input: PlayerInput = serde_json::from_str(&input_string).unwrap();
    game_loop(input.to_game_input(), time_elapsed, delta);
}


#[test]
fn test_loop() {
    fn test_random() -> f32 {
        return 0.5;
    }
    let p = Platform {
        random: test_random,
    };
    game_init(960.0, 540.0, p);
    let g = GameInput {
        up_key: false,
        down_key: false,
        left_key: false,
        right_key: false,
        shoot_right: false,
        shoot_left: false,
        shoot_up: false,
        shoot_down: false,
        quit_key: false,
        space: false,
    };
    let ret = game_loop(g, 0.1, 0.1);
    println!("{:?}", ret);
}