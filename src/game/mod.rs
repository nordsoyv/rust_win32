use entities::Color;
use entities::Entity;
use entities::Force;
use entities::FEATURE_PLAYER;
use std::time::Duration;
use std::time::Instant;

pub struct GameState {
    pub frame: u32,
    pub input: GameInput,
    pub time: GameTime,
    pub entities: Vec<Entity>,
}

pub struct GameInput {
    pub up_key: bool,
    pub down_key: bool,
    pub left_key: bool,
    pub right_key: bool,
    pub quit_key: bool,
}

impl GameInput {
    fn new() -> GameInput {
        GameInput {
            down_key: false,
            left_key: false,
            quit_key: false,
            right_key: false,
            up_key: false,
        }
    }
}

pub struct GameTime {
    pub game_start_time: Instant,
    pub frame_start_time: Instant,
    pub last_frame_time: Duration,
}

impl GameTime {
    fn new() -> GameTime {
        GameTime {
            game_start_time: Instant::now(),
            frame_start_time: Instant::now(),
            last_frame_time: Duration::new(0, 0),
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        let mut entities = Vec::new();
        entities.push(Entity::create_static(
            2.0,
            540.0 / 2.0,
            4.0,
            540.0,
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            Force::Neutral,
        ));

        entities.push(Entity::create_static(
            960.0 - 2.0,
            540.0 / 2.0,
            4.0,
            540.0,
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            Force::Neutral,
        ));

        entities.push(Entity::create_static(
            960.0 / 2.0,
            2.0,
            960.0,
            4.0,
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            Force::Neutral,
        ));
        entities.push(Entity::create_static(
            960.0 / 2.0,
            540.0 - 2.0,
            960.0,
            4.0,
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            Force::Neutral,
        ));

        entities.push(Entity::create_player(
            10.0,
            10.0,
            20.0,
            20.0,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        ));

        GameState {
            input: GameInput::new(),
            frame: 0,
            time: GameTime::new(),
            entities,
        }
    }
}

pub fn game_loop(game_state: &mut GameState) -> bool {
    if game_state.input.quit_key {
        return false;
    }

    let step_size: f32 = 5.0;

    for mut e in &mut game_state.entities {
        if e.features & FEATURE_PLAYER > 0 {
            if game_state.input.up_key {
                e.pos_y += step_size;
            }
            if game_state.input.down_key {
                e.pos_y -= step_size;
            }
            if game_state.input.left_key {
                e.pos_x -= step_size;
            }
            if game_state.input.right_key {
                e.pos_x += step_size;
            }
        }
    }

    //println!("Frame {} ", game_state.frame);
    //println!("Time taken for last frame: {:?}", game_state.last_frame_time);
    //println!("Total time taken {:?}", game_state.game_start_time.elapsed());

    return true;
}
