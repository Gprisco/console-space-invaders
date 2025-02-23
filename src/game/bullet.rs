#[derive(Debug)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub symbol: char,
    pub active: bool,
    velocity: f32, // Negative for upward movement
}

impl Bullet {
    pub fn new(x: f32, y: f32, symbol: char, velocity: f32) -> Self {
        Self {
            x,
            y,
            symbol,
            active: true,
            velocity,
        }
    }

    pub fn update(&mut self) {
        self.y += self.velocity;
    }

    pub fn is_out_of_bounds(&self, height: u16) -> bool {
        self.y < 0.0 || self.y >= height as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullet_movement() {
        let mut bullet = Bullet::new(10.0, 20.0, '|', -1.0);

        bullet.update();
        assert_eq!(bullet.y, 19.0);
    }

    #[test]
    fn test_bullet_bounds() {
        let bullet = Bullet::new(10.0, -1.0, '|', -1.0);
        assert!(bullet.is_out_of_bounds(24));

        let bullet = Bullet::new(10.0, 25.0, '|', 1.0);
        assert!(bullet.is_out_of_bounds(24));
    }
}
