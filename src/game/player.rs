#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub symbol: char,
}

impl Player {
    pub fn new(x: f32, y: f32, symbol: char) -> Self {
        Self { x, y, symbol }
    }

    pub fn move_left(&mut self, speed: f32) {
        self.x -= speed;
    }

    pub fn move_right(&mut self, speed: f32) {
        self.x += speed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_movement() {
        let mut player = Player::new(10.0, 20.0, '▀');

        player.move_left(1.0);
        assert_eq!(player.x, 9.0);

        player.move_right(2.0);
        assert_eq!(player.x, 11.0);
    }
}
