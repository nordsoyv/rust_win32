use entities::BoundingBox;
use entities::Collider;
use entities::Color;
use entities::Drawable;
use entities::Position;
use math::pulse_value;
use math::vector::Vector2d;
use GameTime;

pub struct Bullet {
    pos: Vector2d,
    width: f32,
    height: f32,
    color: Color,
    vel: Vector2d,
    life_time: f32,
}

const VEL: f32 = 300.0;

impl Bullet {
    pub fn new(pos: Vector2d, direction: Vector2d) -> Bullet {
        Bullet {
            pos,
            width: 4.0,
            height: 4.0,
            color: Color {
                r: 1.0,
                g: 0.7,
                b: 0.7,
                a: 1.0,
            },
            life_time: 0.0,
            vel: direction.mul(VEL),
        }
    }

    pub fn update(&mut self, time: &GameTime) {
        self.life_time += time.delta;
        self.pos.x += self.vel.x * time.delta;
        self.pos.y += self.vel.y * time.delta;
        self.color.r = pulse_value(0.7, 1.0, self.life_time * 5.0);
    }
}

impl Position for Bullet {
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

impl Collider for Bullet {
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.pos.x - (self.width / 2.0),
            right: self.pos.x + (self.width / 2.0),
            top: self.pos.y + (self.height / 2.0),
            bottom: self.pos.y - (self.height / 2.0),
        }
    }
}

impl Drawable for Bullet {
    fn get_color(&self) -> &Color {
        &self.color
    }
}
