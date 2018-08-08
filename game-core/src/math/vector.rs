#[derive(Debug, Copy, Clone)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vector2d {
    pub fn new(x: f32, y: f32,) -> Vector2d {
        Vector2d { x, y, }
    }

    pub fn normalize(&mut self) {
        let sum = self.x.powi(2,) + self.y.powi(2,);
        let sqrt = sum.sqrt();
        self.x = self.x / sqrt;
        self.y = self.y / sqrt;
    }

    pub fn set(&mut self, x: f32, y: f32,) {
        self.x = x;
        self.y = y;
    }

    pub fn add(&mut self, other: &Vector2d,) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }

    pub fn sub(&mut self, other: &Vector2d,) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }

    pub fn mul(&self, m: f32,) -> Vector2d {
        Vector2d::new(self.x * m, self.y * m,)
    }

    pub fn len(&self) -> f32 {
        let sum = self.x.powi(2,) + self.y.powi(2,);
        sum.sqrt()
    }
}

#[test]
fn test_norm() {
    let mut v = Vector2d::new(10.0, 10.0,);
    v.normalize();
    assert_eq!(v.x, 0.70710677);
}

#[test]
fn test_set() {
    let mut v = Vector2d::new(10.0, 10.0,);
    v.set(1.0, 2.0,);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
}
