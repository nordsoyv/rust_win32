use std::time::Instant;
use std::time::Duration;


pub struct GameState {
  pub frame: u32,
  pub input: GameInput,
  pub time: GameTime,
  pub player: Player,
}

pub struct GameInput {
  pub up_key: bool,
  pub down_key: bool,
  pub left_key: bool,
  pub right_key: bool,
  pub quit_key: bool,
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      input: GameInput::new(),
      frame: 0,
      time: GameTime::new(),
      player: Player::new(),
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
}

impl Player {
  fn new() -> Player {
    Player {
      pos_x: 0.0,
      pos_y: 0.0,
    }
  }
}

pub fn game_loop(game_state: &mut GameState) -> bool {
  if game_state.input.quit_key {
    return false;
  }


  if game_state.input.up_key {
    game_state.player.pos_y += 1.0;
  }
  if game_state.input.down_key {
    game_state.player.pos_y -= 1.0;
  }
  if game_state.input.left_key {
    game_state.player.pos_x -=1.0 ;
  }
  if game_state.input.right_key {
    game_state.player.pos_x +=1.0 ;
  }

  //println!("Frame {} ", game_state.frame);
  //println!("Time taken for last frame: {:?}", game_state.last_frame_time);
  //println!("Total time taken {:?}", game_state.game_start_time.elapsed());


  return true;
}

