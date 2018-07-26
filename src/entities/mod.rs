pub mod bullet;
pub mod player;
pub mod wall;

use math::vector::Vector2d;

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub trait Position {
    fn get_position(&self) -> Vector2d;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
}

pub trait Collider: Position {
    fn get_bounding_box(&self) -> BoundingBox;
}

pub trait Drawable: Collider {
    fn get_color(&self) -> &Color;
}

pub struct BoundingBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
