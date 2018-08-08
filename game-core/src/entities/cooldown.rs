pub struct Cooldown {
    current: f32,
    cool_down: f32,
}

impl Cooldown {
    pub fn new(time: f32) -> Cooldown {
        Cooldown {
            current: time,
            cool_down: time,
        }
    }

    pub fn set_cooldown(&mut self, new_cooldown: f32,) {
        self.cool_down = new_cooldown;
    }

    pub fn update(&mut self, delta: f32,) {
        self.current -= delta;
    }

    pub fn is_elapsed(&self) -> bool {
        self.current < 0.0
    }

    pub fn restart(&mut self) {
        self.current = self.cool_down;
    }
}

#[test]
fn test_elapsed() {
    let mut c = Cooldown::new(1.0,);
    assert_eq!(c.is_elapsed(), false);
    c.update(1.1,);
    assert_eq!(c.is_elapsed(), true);
    c.restart();
    assert_eq!(c.is_elapsed(), false);
    c.update(0.1,);
    assert_eq!(c.is_elapsed(), false);
    c.update(0.1,);
    assert_eq!(c.is_elapsed(), false);
    c.update(1.1,);
    assert_eq!(c.is_elapsed(), true);
    c.set_cooldown(0.5,);
    c.restart();
    assert_eq!(c.is_elapsed(), false);
    c.update(0.6,);
    assert_eq!(c.is_elapsed(), true);
}
