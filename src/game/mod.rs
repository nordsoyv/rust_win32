use entities::bullet::Bullet;
use entities::player::Player;
use entities::wall::Wall;
use entities::Collider;
use entities::Intersection;
use entities::Position;
use entities::Side;
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
    pub time_elapsed: f32,
    pub delta: f32,
}

impl GameTime {
    fn new() -> GameTime {
        GameTime {
            game_start_time: Instant::now(),
            frame_start_time: Instant::now(),
            last_frame_time: Duration::new(0, 0),
            time_elapsed: 0.0,
            delta: 0.0,
        }
    }
}

pub fn game_loop(game_state: &mut GameState) -> bool {
    if game_state.input.quit_key {
        return false;
    }
    game_state.update();
    return true;
}

impl GameState {
    pub fn update(&mut self) {
        self.update_bullets();
        self.player
            .update(&self.input, &mut self.bullets, self.time.delta);

        let intersections = self.check_intersections();

        self.player.handle_collisions(intersections);
    }

    fn update_bullets(&mut self) -> () {
        let mut bullets_to_delete: Vec<usize> = Vec::new();
        let mut index: usize = 0;

        for b in &mut self.bullets {
            b.update(&self.time);

            let pos = b.get_position();

            if pos.x < 0.0 || pos.x > self.world_size_x || pos.y < 0.0 || pos.y > self.world_size_y
            {
                bullets_to_delete.push(index);
            }
            index += 1;
        }

        if bullets_to_delete.len() > 0 {
            bullets_to_delete.reverse();
            for index_to_delete in bullets_to_delete {
                self.bullets.remove(index_to_delete);
            }
        }
    }

    fn check_intersections(&self) -> Option<Vec<Intersection>> {
        let walls = &self.walls;
        let player = &self.player;
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
