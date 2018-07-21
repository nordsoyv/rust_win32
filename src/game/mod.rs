use entities::Bullet;
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
    pub players: Vec<Entity>,
    pub walls: Vec<Entity>,
    pub bullets: Vec<Bullet>,
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

impl GameInput {
    fn new() -> GameInput {
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
        let mut players = Vec::new();
        let mut walls = Vec::new();
        players.push(Entity::create_player(
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

        walls.push(Entity::create_static(
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

        walls.push(Entity::create_static(
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

        walls.push(Entity::create_static(
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
        walls.push(Entity::create_static(
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
            players,
            walls,
            bullets: Vec::new(),
        }
    }
}

pub fn game_loop(mut game_state: &mut GameState) -> bool {
    move_bullets(&mut game_state);

    let mut player = &mut game_state.players[0];

    if game_state.input.quit_key {
        return false;
    }

    move_player(&game_state.input, player);
    if game_state.input.shoot_right {
        game_state.bullets.push(Entity::create_bullet(
            player.pos_x,
            player.pos_y,
            2.0,
            2.0,
            100.0,
            0.0,
            Color {
                r: 1.0,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
        ))
    }

    let mut intersections = None;

    intersections = check_intersections(player, &game_state.walls);

    match intersections {
        Some(inter) => {
            for i in inter {
                match i.hit_side {
                    Side::Left => {
                        player.pos_x += i.amount;
                    }
                    Side::Right => {
                        player.pos_x -= i.amount;
                    }
                    Side::Top => {
                        player.pos_y -= i.amount;
                    }
                    Side::Bottom => {
                        player.pos_y += i.amount;
                    }
                }
            }
        } //println!("Got intersections : {}", inter.len()),
        None => {}
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
    entity1: u32,
    entity2: u32,
    hit_side: Side,
    amount: f32,
}

fn check_intersections(player: &Entity, walls: &Vec<Entity>) -> Option<Vec<Intersection>> {
    let mut results = Vec::new();
    for mut other_e in &mut walls.iter() {
        let intersection = check_intersection(player, other_e);
        match intersection {
            Some(inter) => {
                //println!("{:?}", inter);
                results.push(inter)
            }
            None => {}
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
                entity1: player.id,
                entity2: other.id,
                hit_side: Side::Left,
                amount: left_side_intersection * -1.0,
            });
        }

        if right_side_intersection >= left_side_intersection
            && right_side_intersection >= top_side_intersection
            && right_side_intersection >= bottom_side_intersection
        {
            return Some(Intersection {
                entity1: player.id,
                entity2: other.id,
                hit_side: Side::Right,
                amount: right_side_intersection * -1.0,
            });
        }

        if top_side_intersection >= left_side_intersection
            && top_side_intersection >= right_side_intersection
            && top_side_intersection >= bottom_side_intersection
        {
            return Some(Intersection {
                entity1: player.id,
                entity2: other.id,
                hit_side: Side::Top,
                amount: top_side_intersection * -1.0,
            });
        }

        if bottom_side_intersection >= left_side_intersection
            && bottom_side_intersection >= top_side_intersection
            && bottom_side_intersection >= right_side_intersection
        {
            return Some(Intersection {
                entity1: player.id,
                entity2: other.id,
                hit_side: Side::Bottom,
                amount: bottom_side_intersection * -1.0,
            });
        }
    }
    None
}

fn move_bullets(game_state: &mut GameState) -> () {
    let mut delta = game_state.time.last_frame_time.subsec_micros() as f32;
    delta = delta / (1000.0 * 1000.0);

    println!("delta {}", delta);

    for b in &mut game_state.bullets {
        b.pos_x += b.vel_x * delta;
    }
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
