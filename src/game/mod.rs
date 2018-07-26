use entities::bullet::Bullet;
use entities::Collider;
//use entities::Bullet;
use entities::player::Player;
use entities::wall::Wall;
use entities::Position;
use math::vector::Vector2d;
use std::time::Duration;
use std::time::Instant;

pub struct GameState {
    pub frame: u32,
    pub input: GameInput,
    pub time: GameTime,
    pub player: Player,
    pub walls: Vec<Wall>,
    pub bullets: Vec<Bullet>,
    pub world_size_x: f32,
    pub world_size_y: f32,
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
    pub delta: f32,
}

impl GameTime {
    fn new() -> GameTime {
        GameTime {
            game_start_time: Instant::now(),
            frame_start_time: Instant::now(),
            last_frame_time: Duration::new(0, 0),
            delta: 0.0,
        }
    }
}

impl GameState {
    pub fn new(size_x: f32, size_y: f32) -> GameState {
        let player = Player::new();
        let mut walls = Vec::new();

        walls.push(Wall::new(Vector2d::new(2.0, 540.0 / 2.0), 4.0, 540.0));
        walls.push(Wall::new(
            Vector2d::new(960.0 - 2.0, 540.0 / 2.0),
            4.0,
            540.0,
        ));
        walls.push(Wall::new(Vector2d::new(960.0 / 2.0, 2.0), 960.0, 4.0));
        walls.push(Wall::new(
            Vector2d::new(960.0 / 2.0, 540.0 - 2.0),
            960.0,
            4.0,
        ));

        GameState {
            input: GameInput::new(),
            frame: 0,
            time: GameTime::new(),
            player,
            walls,
            bullets: Vec::new(),
            world_size_x: size_x,
            world_size_y: size_y,
        }
    }
}

pub fn game_loop(mut game_state: &mut GameState) -> bool {
    if game_state.input.quit_key {
        return false;
    }

    update_bullets(&mut game_state);
    game_state.player.update(&game_state.input);
    fire_bullets(&mut game_state);

    let intersections = check_intersections(&game_state);
    //    let mut player = &mut game_state.players[0];

    handle_collisions(&mut game_state.player, intersections);

    //println!("Frame {} ", game_state.frame);
    //println!("Time taken for last frame: {:?}", game_state.last_frame_time);
    //println!("Total time taken {:?}", game_state.game_start_time.elapsed());

    return true;
}

fn handle_collisions(player: &mut Player, intersections: Option<Vec<Intersection>>) {
    let player_pos = player.get_position();
    match intersections {
        Some(inter) => {
            for i in inter {
                match i.hit_side {
                    Side::Left => {
                        player.set_x(player_pos.x + i.amount);
                    }
                    Side::Right => {
                        player.set_x(player_pos.x - i.amount);
                    }
                    Side::Top => {
                        player.set_y(player_pos.y + i.amount);
                    }
                    Side::Bottom => {
                        player.set_y(player_pos.y - i.amount);
                    }
                }
            }
        } //println!("Got intersections : {}", inter.len()),
        None => {}
    }
}


fn fire_bullets(game_state: &mut &mut GameState) {
    let player = &game_state.player;

    let mut direction = Vector2d { x: 0.0, y: 0.0 };

    if game_state.input.shoot_right {
        direction.x += 1.0;
    }
    if game_state.input.shoot_left {
        direction.x -= 1.0;
    }
    if game_state.input.shoot_up {
        direction.y += 1.0;
    }
    if game_state.input.shoot_down {
        direction.y -= 1.0;
    }
    if direction.len() > 0.5 {
        let bullet = Bullet::new(player.get_position(), direction);
        game_state.bullets.push(bullet);

    }

    //    if vel_y != 0.0 || vel_x != 0.0 {
    //        game_state.bullets.push(Entity::create_bullet(
    //            player.pos_x,
    //            player.pos_y,
    //            2.0,
    //            2.0,
    //            vel_x,
    //            vel_y,
    //            Color {
    //                r: 1.0,
    //                g: 0.1,
    //                b: 0.1,
    //                a: 1.0,
    //            },
    //        ))
    //    };
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
    hit_side: Side,
    amount: f32,
}

fn check_intersections(gs: &GameState) -> Option<Vec<Intersection>> {
    let walls = &gs.walls;
    let player = &gs.player;
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

fn check_intersection(player: &Collider, other: &Collider) -> Option<Intersection> {
    let player_bb = player.get_bounding_box();
    let other_bb = other.get_bounding_box();
    let left_side_intersection = player_bb.left - other_bb.right;
    let right_side_intersection = other_bb.left - player_bb.right;
    let top_side_intersection = other_bb.bottom - player_bb.top;
    let bottom_side_intersection = player_bb.bottom - other_bb.top;

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
                hit_side: Side::Left,
                amount: left_side_intersection * -1.0,
            });
        }

        if right_side_intersection >= left_side_intersection
            && right_side_intersection >= top_side_intersection
            && right_side_intersection >= bottom_side_intersection
        {
            return Some(Intersection {
                hit_side: Side::Right,
                amount: right_side_intersection * -1.0,
            });
        }

        if top_side_intersection >= left_side_intersection
            && top_side_intersection >= right_side_intersection
            && top_side_intersection >= bottom_side_intersection
        {
            return Some(Intersection {
                hit_side: Side::Top,
                amount: top_side_intersection * -1.0,
            });
        }

        if bottom_side_intersection >= left_side_intersection
            && bottom_side_intersection >= top_side_intersection
            && bottom_side_intersection >= right_side_intersection
        {
            return Some(Intersection {
                hit_side: Side::Bottom,
                amount: bottom_side_intersection * -1.0,
            });
        }
    }
    None
}

fn update_bullets(game_state: &mut GameState) -> () {
    let mut bullets_to_delete: Vec<usize> = Vec::new();
    let mut index: usize = 0;

    for b  in &mut game_state.bullets {

        b.update(game_state.time.delta);

        let pos = b.get_position();

        if pos.x < 0.0
            || pos.x > game_state.world_size_x
            || pos.y < 0.0
            || pos.y > game_state.world_size_y
        {
            bullets_to_delete.push(index);
        }
        index += 1;
    }

    if bullets_to_delete.len() > 0 {
        bullets_to_delete.reverse();
        for index_to_delete in bullets_to_delete {
            game_state.bullets.remove(index_to_delete);
        }
    }
}
