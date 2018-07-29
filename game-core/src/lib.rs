extern crate rand;

use entities::BoundingBox;
use entities::Color;
use game_state::GameState;

pub mod entities;
mod game_state;

mod math;

pub struct Renderable {
    pub rect: BoundingBox,
    pub color: Color,
}

pub struct GameInput {
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

static mut GAME_STATE: Option<GameState> = None;

pub fn game_init(size_x: f32, size_y: f32) {
    unsafe {
        GAME_STATE = Some(GameState::new(size_x, size_y))
    }
}

pub fn game_loop(input: GameInput, time_elapsed: f32, delta: f32) -> Vec<Renderable> {

    unsafe {
        assert!(GAME_STATE.is_some());
        match GAME_STATE {
            Some(ref mut gs) => {
                if input.quit_key {
                    return Vec::new();
                }
                return gs.update(input,time_elapsed, delta);
            }
            None => {return Vec::new(); }
        }

    }
}

impl GameInput {
    pub fn new() -> GameInput {
        GameInput {
            down_key: false,
            left_key: false,
            right_key: false,
            up_key: false,
            shoot_right: false,
            shoot_left: false,
            shoot_up: false,
            shoot_down: false,
            quit_key: false,
            space: false,
        }
    }
}


struct GameTime {
    pub time_elapsed: f32,
    pub delta: f32,
}

impl GameTime {
    fn new() -> GameTime {
        GameTime {
            time_elapsed: 0.0,
            delta: 0.0,
        }
    }
}



