use draw_rectangle;
use end_frame;
use entities::bullet::Bullet;
use entities::Collider;
use entities::cooldown::Cooldown;
use entities::Drawable;
use entities::enemies::Enemy;
use entities::enemies::EnemyType;
use entities::Intersection;
use entities::player::Player;
use entities::Position;
use entities::Side;
use entities::wall::Wall;
use GameInput;
use GameTime;
use get_random;
use math::vector::Vector2d;
use start_frame;

pub struct GameState {
    frame: u32,
    time: GameTime,
    player: Player,
    walls: Vec<Wall>,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    enemy_spawn: Cooldown,
    world_size_x: f32,
    world_size_y: f32,
}

impl GameState {
    pub fn update(&mut self, input: GameInput, time_elapsed: f32, delta: f32) {
        self.frame += 1;
        self.time.time_elapsed = time_elapsed;
        self.time.delta = delta;
        self.update_enemy_spawn();
        self.update_bullets();
        self.update_enemies();
        self.player
            .update(&input, &mut self.bullets, self.time.delta);

        let intersections = self.check_player_walls_intersections();
        self.player.handle_collisions(intersections);

        self.check_bullets_enemies_intersections();

        start_frame();
        for b in &self.bullets {
            let rect = b.get_bounding_box();
            draw_rectangle(rect.left, rect.bottom, rect.right, rect.top, b.get_color());
        }
        {
            let rect = self.player.get_bounding_box();
            draw_rectangle(rect.left, rect.bottom, rect.right, rect.top, self.player.get_color());
        }
        for e in &self.enemies {
            let rect = e.get_bounding_box();
            draw_rectangle(rect.left, rect.bottom, rect.right, rect.top, e.get_color());
        }
        for w in &self.walls {
            let rect = w.get_bounding_box();
            draw_rectangle(rect.left, rect.bottom, rect.right, rect.top, w.get_color());
        }
        end_frame();
    }

    fn update_enemy_spawn(&mut self) {
        self.enemy_spawn.update(self.time.delta);
        if self.enemy_spawn.is_elapsed() {
            self.spawn_enemy();
            self.enemy_spawn.restart();
        }
    }

    fn spawn_enemy(&mut self) {
        let mut x = get_random(5.0, self.world_size_x - 5.0);
        let mut y = get_random(5.0, self.world_size_y - 5.0);
        let mut rand_pos = Vector2d::new(x as f32, y as f32);
        rand_pos.sub(&self.player.get_position());

        while rand_pos.len() < 100.0 {
            x = get_random(5.0, self.world_size_x - 5.0);
            y = get_random(5.0, self.world_size_y - 5.0);
            rand_pos = Vector2d::new(x as f32, y as f32);
            rand_pos.sub(&self.player.get_position());
        }

        self.enemies.push(Enemy::new(
            EnemyType::Normal,
            Vector2d::new(x as f32, y as f32),
        ));
    }

    fn update_enemies(&mut self) {
        for e in &mut self.enemies {
            e.update(&self.player, self.time.delta);
        }
    }

    fn update_bullets(&mut self) -> () {
        let mut bullets_to_delete: Vec<usize> = Vec::new();
        let mut index: usize = 0;

        for b in &mut self.bullets {
            b.update(self.time.delta);

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

    fn check_player_walls_intersections(&self) -> Option<Vec<Intersection>> {
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

    fn check_bullets_enemies_intersections(&mut self) {
        let mut enemy_index: usize;
        let mut bullets_index: usize = 0;
        let mut enemies_to_kill = Vec::new();
        let mut bullets_to_kill = Vec::new();
        for b in &self.bullets {
            enemy_index = 0;
            for e in &self.enemies {
                let intersection = check_intersection(b, e);
                match intersection {
                    Some(_) => {
                        enemies_to_kill.push(enemy_index);
                        bullets_to_kill.push(bullets_index);
                    }
                    None => {}
                }
                enemy_index += 1;
            }
            bullets_index += 1;
        }
        if enemies_to_kill.len() > 0 {
            enemies_to_kill.dedup();
            enemies_to_kill.reverse();
            for index_to_delete in enemies_to_kill {
                self.enemies.remove(index_to_delete);
            }
        }
        if bullets_to_kill.len() > 0 {
            bullets_to_kill.dedup();
            bullets_to_kill.reverse();
            for index_to_delete in bullets_to_kill {
                self.bullets.remove(index_to_delete);
            }
        }
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

        let mut enemies = Vec::new();

        enemies.push(Enemy::new(
            EnemyType::Normal,
            Vector2d::new(100.0, 100.0),
        ));
        GameState {
            frame: 0,
            time: GameTime::new(),
            player,
            walls,
            bullets: Vec::new(),
            enemies,
            enemy_spawn: Cooldown::new(0.25),
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
