use entities::BoundingBox;
use entities::Collider;
use entities::Color;
use entities::Drawable;
use entities::player::Player;
use entities::Position;
use math::pulse_value;
use math::vector::Vector2d;

pub enum EnemyType {
    Normal,
}

pub struct Enemy {
    enemy_type: EnemyType,
    pos: Vector2d,
    width: f32,
    height: f32,
    color: Color,
    life_time: f32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, pos: Vector2d) -> Enemy {
        Enemy {
            enemy_type,
            pos,
            life_time: 0.0,
            width: 10.0,
            height: 10.0,
            color: Color {
                r: 0.1,
                g: 1.0,
                b: 0.1,
                a: 11.0,
            },
        }
    }

    pub fn update(&mut self, player: &Player, delta: f32) {
        self.life_time += delta;
        match self.enemy_type {
            EnemyType::Normal => {
                let player_pos = player.get_position();
                let mut current_pos = self.pos;
                current_pos.sub(&player_pos);
                current_pos.normalize();
                current_pos = current_pos.mul(-1.0);
                self.pos.add(&current_pos);
                self.width = 10.0 + pulse_value(0.0, 5.0, self.life_time * 10.0);
                self.height = 10.0 + pulse_value(0.0, 5.0, self.life_time * 7.5);
            }
        }
    }
}

impl Position for Enemy {
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

impl Collider for Enemy {
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.pos.x - (self.width / 2.0),
            right: self.pos.x + (self.width / 2.0),
            top: self.pos.y + (self.height / 2.0),
            bottom: self.pos.y - (self.height / 2.0),
        }
    }
}

impl Drawable for Enemy {
    fn get_color(&self) -> Color {
        Color {
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: self.color.a,
        }
    }
}
