#[derive(Debug)]
pub struct Alien {
    pub x: f32,
    pub y: f32,
    pub symbol: char,
    pub direction: f32, // 1.0 for right, -1.0 for left
    pub alive: bool,
}

impl Alien {
    pub fn new(x: f32, y: f32, symbol: char) -> Self {
        Self {
            x,
            y,
            symbol,
            direction: 1.0,
            alive: true,
        }
    }

    pub fn update(&mut self, speed: f32) {
        self.x += speed * self.direction;
    }

    pub fn reverse_direction(&mut self, drop_amount: f32) {
        self.direction *= -1.0;
        self.y += drop_amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alien_movement() {
        let mut alien = Alien::new(10.0, 5.0, '👾');

        alien.update(1.0);
        assert_eq!(alien.x, 11.0);

        alien.reverse_direction(1.0);
        alien.update(1.0);
        assert_eq!(alien.x, 10.0);
        assert_eq!(alien.y, 6.0);
    }
}
