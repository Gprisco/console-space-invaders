use super::alien::Alien;
use super::bullet::Bullet;
use super::player::Player;
use crate::config::GameConfig;
use crate::input::GameInput;

pub enum GameStatus {
    Playing,
    Paused,
    GameOver,
}

pub struct GameState {
    pub player: Player,
    pub aliens: Vec<Alien>,
    pub bullets: Vec<Bullet>,
    pub score: u32,
    pub status: GameStatus,
    config: GameConfig,
    frame_count: u32,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        let player = Player::new(
            config.window_width as f32 / 2.0,
            config.window_height as f32 - 2.0,
            config.player_symbol,
        );

        let mut aliens = Vec::new();
        for row in 0..config.alien_rows {
            for col in 0..config.alien_columns {
                aliens.push(Alien::new(
                    (col * 2 + 2) as f32,
                    (row * 2 + 1) as f32,
                    config.alien_symbol,
                ));
            }
        }

        Self {
            player,
            aliens,
            bullets: Vec::new(),
            score: 0,
            status: GameStatus::Playing,
            config,
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
            if bullet.is_out_of_bounds(self.config.window_height) {
                bullet.active = false;
            }
        }
        self.bullets.retain(|bullet| bullet.active);

        // Update aliens every N frames
        if self.frame_count % 30 == 0 {
            let mut should_reverse = false;

            // Check if any alien would hit the boundaries
            for alien in &self.aliens {
                if alien.alive {
                    let next_x = alien.x + self.config.alien_speed * alien.direction;
                    if next_x <= 0.0 || next_x >= self.config.window_width as f32 - 1.0 {
                        should_reverse = true;
                        break;
                    }
                }
            }

            // Update alien positions
            for alien in &mut self.aliens {
                if alien.alive {
                    if should_reverse {
                        alien.reverse_direction(1.0);
                    } else {
                        alien.update(self.config.alien_speed);
                    }
                }
            }
        }

        // Check collisions
        self.check_collisions();
    }

    fn check_collisions(&mut self) {
        for bullet in &mut self.bullets {
            if !bullet.active {
                continue;
            }

            for alien in &mut self.aliens {
                if !alien.alive {
                    continue;
                }

                if (bullet.x - alien.x).abs() < 1.0 && (bullet.y - alien.y).abs() < 1.0 {
                    bullet.active = false;
                    alien.alive = false;
                    self.score += 10;
                }
            }
        }
    }

    pub fn handle_input(&mut self, input: GameInput) {
        match input {
            GameInput::Left => {
                self.player.move_left(self.config.player_speed);
            }
            GameInput::Right => {
                self.player.move_right(self.config.player_speed);
            }
            GameInput::Fire => {
                self.bullets.push(Bullet::new(
                    self.player.x,
                    self.player.y - 1.0,
                    self.config.bullet_symbol,
                    -self.config.bullet_speed,
                ));
            }
            _ => {}
        }

        // Ensure player stays within bounds
        self.player.x = self
            .player
            .x
            .clamp(0.0, self.config.window_width as f32 - 1.0);
    }
}
