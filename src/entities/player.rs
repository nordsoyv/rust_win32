use entities::BoundingBox;
use entities::Collider;
use entities::Color;
use entities::Position;
use game::GameInput;
use math::vector::Vector2d;
use entities::Drawable;

pub struct Player {
    pos: Vector2d,
    width: f32,
    height: f32,
    color: Color,
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
        }
    }

    pub fn update(&mut self, input: &GameInput) {
        let mut step_size: f32 = 1.0;
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
    fn get_color(&self) -> &Color {
        &self.color
    }
}