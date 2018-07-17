#[derive(Debug)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

pub struct Entity {
  pub features: u64,
  pub pos_x: f32,
  pub pos_y: f32,
  pub width: f32,
  pub height: f32,
  pub color: Color,
}

pub static FEATURE_PLAYER: u64 = 1 << 0;
pub static FEATURE_DRAWABLE: u64 = 1 << 1;


impl Entity {
  pub fn create_static(x: f32, y: f32, width: f32, height: f32, color: Color) -> Entity {
    Entity {
      features: FEATURE_DRAWABLE,
      pos_x: x,
      pos_y: y,
      width,
      height,
      color,
    }
  }

  pub fn create_player(x: f32, y: f32, width: f32, height: f32, color: Color) -> Entity {
    Entity {
      features: FEATURE_DRAWABLE | FEATURE_PLAYER,
      pos_x: x,
      pos_y: y,
      width,
      height,
      color,
    }
  }

}
