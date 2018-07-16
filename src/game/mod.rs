use std::time::Duration;
use std::time::Instant;

pub struct GameState {
  pub frame: u32,
  pub input: GameInput,
  pub time: GameTime,
  pub player: Player,
  pub entities: Vec<Entity>,
}

pub struct GameInput {
  pub up_key: bool,
  pub down_key: bool,
  pub left_key: bool,
  pub right_key: bool,
  pub quit_key: bool,
}

#[derive(Debug)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}


impl GameState {
  pub fn new() -> GameState {
    let mut entities = Vec::new();
    entities.push(Entity {
      pos_x: 2.0,
      pos_y: 540.0 / 2.0,
      width: 4.0,
      height: 540.0,
      color: Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
      },
    });

    entities.push(Entity {
      pos_x: 960.0 - 2.0,
      pos_y: 540.0 / 2.0,
      width: 4.0,
      height: 540.0,
      color: Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
      },
    });


    entities.push(Entity {
      pos_x: 960.0 / 2.0,
      pos_y: 2.0,
      width: 960.0,
      height: 4.0,
      color: Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
      },
    });
    entities.push(Entity {
      pos_x: 960.0 / 2.0,
      pos_y: 540.0 - 2.0,
      width: 960.0,
      height: 4.0,
      color: Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
      },
    });


    GameState {
      input: GameInput::new(),
      frame: 0,
      time: GameTime::new(),
      player: Player::new(),
      entities,
    }
  }
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

#[derive(Debug)]
pub struct Player {
  pub pos_x: f32,
  pub pos_y: f32,
  pub color : Color,
}

impl Player {
  fn new() -> Player {
    Player {
      pos_x: 0.0,
      pos_y: 0.0,
      color : Color{
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
      }
    }
  }
}

pub struct Entity {
  pub pos_x: f32,
  pub pos_y: f32,
  pub width: f32,
  pub height: f32,
  pub color: Color,
}


pub fn game_loop(game_state: &mut GameState) -> bool {
  if game_state.input.quit_key {
    return false;
  }

  let step_size:f32 = 5.0 ;

  if game_state.input.up_key {
    game_state.player.pos_y += step_size;
  }
  if game_state.input.down_key {
    game_state.player.pos_y -= step_size;
  }
  if game_state.input.left_key {
    game_state.player.pos_x -= step_size;
  }
  if game_state.input.right_key {
    game_state.player.pos_x += step_size;
  }

  //println!("Frame {} ", game_state.frame);
  //println!("Time taken for last frame: {:?}", game_state.last_frame_time);
  //println!("Total time taken {:?}", game_state.game_start_time.elapsed());


  return true;
}

