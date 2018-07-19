use entities::Color;
use entities::Entity;
use entities::Force;
use entities::{FEATURE_COLLIDABLE, FEATURE_PLAYER};
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
    pub space: bool,
}

impl GameInput {
    fn new() -> GameInput {
        GameInput {
            down_key: false,
            left_key: false,
            quit_key: false,
            right_key: false,
            up_key: false,
            space: false,
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

    for mut e in &mut game_state.entities {
        if e.has_feature(FEATURE_PLAYER) {
            move_player(&game_state.input, e);
        }
    }

    for e in &game_state.entities {
        if e.has_feature(FEATURE_PLAYER) {
            let i = check_intersections(&e, &game_state.entities);
            match i {
        Some(inter) => {
          for i in inter {
            match i.hit_side {
              Side::Left => {        }
              Side::Right => {              }
              Side::Top => {              }
              Side::Bottom => {              }
            }
          }
        } //println!("Got intersections : {}", inter.len()),
        None => {}
      }
        }
    }

    //println!("Frame {} ", game_state.frame);
    //println!("Time taken for last frame: {:?}", game_state.last_frame_time);
    //println!("Total time taken {:?}", game_state.game_start_time.elapsed());

    return true;
}
#[derive(Debug)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
struct Intersection {
    hit_id: u32,
    hit_side: Side,
    amount: f32,
}

fn check_intersections(player: &Entity, entities: &Vec<Entity>) -> Option<Vec<Intersection>> {
    let mut results = Vec::new();
    for mut other_e in &mut entities.iter() {
        if other_e.has_feature(FEATURE_COLLIDABLE) {
            let intersection = check_intersection(player, other_e);
            match intersection {
                Some(inter) => {
                    println!("{:?}", inter);
                    results.push(inter)
                }
                None => {}
            }
        }
    }
    if results.len() > 0 {
        return Some(results);
    }
    return None;
}

fn check_intersection(player: &Entity, other: &Entity) -> Option<Intersection> {
    let player_left = player.pos_x - (player.width / 2.0);
    let player_right = player.pos_x + (player.width / 2.0);
    let player_top = player.pos_y + (player.height / 2.0);
    let player_bottom = player.pos_y - (player.height / 2.0);

    let other_left = other.pos_x - (other.width / 2.0);
    let other_right = other.pos_x + (other.width / 2.0);
    let other_top = other.pos_y + (other.height / 2.0);
    let other_bottom = other.pos_y - (other.height / 2.0);

    let left_side_intersection = player_left - other_right;
    let right_side_intersection = other_left - player_right;
    let top_side_intersection = other_bottom - player_top;
    let bottom_side_intersection = player_bottom - other_top;

    if left_side_intersection < 0.0
        && right_side_intersection < 0.0
        && top_side_intersection < 0.0
        && bottom_side_intersection < 0.0
    {
        if left_side_intersection >= right_side_intersection
            && left_side_intersection >= top_side_intersection
            && left_side_intersection >= bottom_side_intersection
        {
            return Some(Intersection {
                hit_id: other.id,
                hit_side: Side::Left,
                amount: left_side_intersection * -1.0,
            });
        }

        if right_side_intersection >= left_side_intersection
            && right_side_intersection >= top_side_intersection
            && right_side_intersection >= bottom_side_intersection
        {
            return Some(Intersection {
                hit_id: other.id,
                hit_side: Side::Right,
                amount: right_side_intersection * -1.0,
            });
        }

        if top_side_intersection >= left_side_intersection
            && top_side_intersection >= right_side_intersection
            && top_side_intersection >= bottom_side_intersection
        {
            return Some(Intersection {
                hit_id: other.id,
                hit_side: Side::Top,
                amount: top_side_intersection * -1.0,
            });
        }

        if bottom_side_intersection >= left_side_intersection
            && bottom_side_intersection >= top_side_intersection
            && bottom_side_intersection >= right_side_intersection
        {
            return Some(Intersection {
                hit_id: other.id,
                hit_side: Side::Bottom,
                amount: bottom_side_intersection * -1.0,
            });
        }
    }
    None
}

fn move_player(input: &GameInput, e: &mut Entity) -> () {
    let mut step_size: f32 = 1.0;
    if input.space {
        step_size = 10.0;
    }
    if input.up_key {
        e.pos_y += step_size;
    }
    if input.down_key {
        e.pos_y -= step_size;
    }
    if input.left_key {
        e.pos_x -= step_size;
    }
    if input.right_key {
        e.pos_x += step_size;
    }
}
