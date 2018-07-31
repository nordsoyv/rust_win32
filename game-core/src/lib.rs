use entities::BoundingBox;
use entities::Color;
use game_state::GameState;

pub mod entities;
mod game_state;

mod math;

#[derive(Debug)]
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

pub struct Platform {
    pub random: fn() -> f32,
    pub log: fn(s:String)
}

static mut GAME_STATE: Option<GameState> = None;
static mut PLATFORM: Option<Platform> = None;

pub fn game_init(size_x: f32, size_y: f32, platform: Platform) {
    unsafe {
        PLATFORM = Some(platform);
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

                return gs.update(input, time_elapsed, delta);
            }
            None => { return Vec::new(); }
        }
    }
}

pub fn get_random(min: f32, max: f32) -> f32 {
    unsafe {
        match PLATFORM {
            Some(ref pf) => {
                return (pf.random)() * (max - min) + min;
            }
            None => {
                return 0.0;
            }
        }
    }
}

pub fn log(text: String) {
    unsafe {
        match PLATFORM {
            Some(ref pf) => {
                (pf.log)(text);
            }
            None => {}
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



