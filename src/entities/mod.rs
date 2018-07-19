pub enum Force {
    Neutral,
    Player,
    Enemy,
}

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct Entity {
    pub id: u32,
    pub features: u64,
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub force: Force,
}

pub static FEATURE_PLAYER: u64 = 1 << 0;
pub static FEATURE_DRAWABLE: u64 = 1 << 1;
pub static FEATURE_COLLIDABLE: u64 = 1 << 2;

static mut CURR_ID: u32 = 0;

fn next_id() -> u32 {
    unsafe {
        let next = CURR_ID;
        CURR_ID += 1;
        println!("Creating id {}", next);
        next
    }
}

impl Entity {
    pub fn has_feature(&self, feature: u64) -> bool {
        if self.features & feature != 0 {
            return true;
        }
        false
    }

    pub fn create_static(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        force: Force,
    ) -> Entity {
        Entity {
            id: next_id(),
            force,
            features: FEATURE_DRAWABLE | FEATURE_COLLIDABLE,
            pos_x: x,
            pos_y: y,
            width,
            height,
            color,
        }
    }

    pub fn create_player(x: f32, y: f32, width: f32, height: f32, color: Color) -> Entity {
        Entity {
            id: next_id(),
            force: Force::Player,
            features: FEATURE_DRAWABLE | FEATURE_PLAYER,
            pos_x: x,
            pos_y: y,
            width,
            height,
            color,
        }
    }
}
