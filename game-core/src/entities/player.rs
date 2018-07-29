use entities::BoundingBox;
use entities::bullet::Bullet;
use entities::Collider;
use entities::Color;
use entities::cooldown::Cooldown;
use entities::Drawable;
use entities::Intersection;
use entities::Position;
use entities::Side;
use GameInput;
use math::vector::Vector2d;

pub struct Player {
    pos: Vector2d,
    width: f32,
    height: f32,
    color: Color,
    shoot_cooldown: Cooldown,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector2d {
                x: 960.0 / 2.0,
                y: 540.0 / 2.0,
            },
            width: 10.0,
            height: 10.0,
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            shoot_cooldown: Cooldown::new(0.1),
        }
    }

    pub fn update(&mut self, input: &GameInput, bullets: &mut Vec<Bullet>, delta: f32) {
        self.shoot_cooldown.update(delta);
        self.update_pos(&input);
        self.fire_bullets(&input, bullets);
    }

    fn update_pos(&mut self, input: &GameInput) {
        let mut step_size: f32 = 1.5;
        if input.space {
            step_size = 10.0;
        }
        if input.up_key {
            self.pos.y += step_size;
        }
        if input.down_key {
            self.pos.y -= step_size;
        }
        if input.left_key {
            self.pos.x -= step_size;
        }
        if input.right_key {
            self.pos.x += step_size;
        }
    }

    fn fire_bullets(&mut self, input: &GameInput, bullets: &mut Vec<Bullet>) {
        let mut direction = Vector2d { x: 0.0, y: 0.0 };
        if self.shoot_cooldown.is_elapsed() {
            self.shoot_cooldown.restart();
            if input.shoot_right {
                direction.x += 1.0;
            }
            if input.shoot_left {
                direction.x -= 1.0;
            }
            if input.shoot_up {
                direction.y += 1.0;
            }
            if input.shoot_down {
                direction.y -= 1.0;
            }
            if direction.len() > 0.5 {
                let bullet = Bullet::new(self.get_position(), direction);
                bullets.push(bullet);
            }
        }
    }

    pub fn handle_collisions(&mut self, intersections: Option<Vec<Intersection>>) {
        let player_pos = self.get_position();

        match intersections {
            Some(inter) => {
                for i in inter {
                    match i.hit_side {
                        Side::Left => {
                            self.set_x(player_pos.x + i.amount);
                        }
                        Side::Right => {
                            self.set_x(player_pos.x - i.amount);
                        }
                        Side::Top => {
                            self.set_y(player_pos.y - i.amount);
                        }
                        Side::Bottom => {
                            self.set_y(player_pos.y + i.amount);
                        }
                    }
                }
            }
            None => {}
        }
    }
}

impl Position for Player {
    fn get_position(&self) -> Vector2d {
        self.pos
    }

    fn set_x(&mut self, x: f32) {
        self.pos.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.pos.y = y;
    }
}

impl Collider for Player {
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.pos.x - (self.width / 2.0),
            right: self.pos.x + (self.width / 2.0),
            top: self.pos.y + (self.height / 2.0),
            bottom: self.pos.y - (self.height / 2.0),
        }
    }
}

impl Drawable for Player {
    fn get_color(&self) -> Color {
        Color {
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: self.color.a,
        }
    }
}
