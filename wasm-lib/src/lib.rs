#![feature(use_extern_macros)]
extern crate wasm_bindgen;
extern crate game_core;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;


use wasm_bindgen::prelude::*;
use game_core::game_init;
use game_core::game_loop;
use game_core::GameInput;
use game_core::Renderable;


#[wasm_bindgen]
extern {
    fn alert(s : &str);
}

#[derive(Serialize,Deserialize,Debug)]
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
        GameInput{
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

#[derive(Serialize,Deserialize,Debug)]
pub struct Rectangle {
    pub left:f32,
    pub right: f32,
    pub top : f32,
    pub bottom : f32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,

}


impl Rectangle {
    fn from_renderable( r : Renderable) -> Rectangle {
        Rectangle {
            left: r.rect.left,
            right: r.rect.right,
            top: r.rect.top,
            bottom: r.rect.bottom,
            red: (r.color.r * 255.0) as u8,
            green: (r.color.g * 255.0) as u8,
            blue: (r.color.b * 255.0) as u8,
        }
    }
}


#[wasm_bindgen]
pub fn greet(name : &str) {
    alert(&format!("Hello!! , {}", name));
}

#[wasm_bindgen]
pub fn init() {
    game_init(960.0, 540.0);
}

#[wasm_bindgen]
pub fn update(input_string: String, time_elapsed :f32, delta : f32 ) ->String {
    let input: PlayerInput = serde_json::from_str(&input_string).unwrap();

    let res = game_loop(input.to_game_input(), time_elapsed, delta);
    let mut ret = Vec::new();
    for r in res {
        ret.push(Rectangle::from_renderable(r))
    }
    let serialized =  serde_json::to_string(&ret).unwrap();
    return serialized;
}