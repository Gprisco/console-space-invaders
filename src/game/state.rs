use super::alien::Alien;
use super::bullet::Bullet;
use super::player::Player;
use crate::config::GameConfig;
use crate::input::GameInput;

const WAVE_CLEARED_BANNER: &str = "WAVE CLEARED";

#[allow(dead_code)]
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
    #[allow(dead_code)]
    pub status: GameStatus,
    config: GameConfig,
    wave_banner_frames: u32,
    banner_duration: u32,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        let player = Player::new(
            config.window_width as f32 / 2.0,
            config.window_height as f32 - 2.0,
            config.player_symbol,
        );

        Self {
            player,
            aliens: spawn_grid(&config),
            bullets: Vec::new(),
            score: 0,
            status: GameStatus::Playing,
            banner_duration: config.fps.saturating_mul(3) / 2,
            config,
            wave_banner_frames: 0,
        }
    }

    pub fn update(&mut self) {
        for bullet in &mut self.bullets {
            bullet.update();
            if bullet.is_out_of_bounds(self.config.window_height) {
                bullet.active = false;
            }
        }
        self.bullets.retain(|bullet| bullet.active);

        self.check_collisions();

        if self.wave_banner_frames > 0 {
            self.wave_banner_frames -= 1;
            if self.wave_banner_frames == 0 {
                self.spawn_wave();
            }
        } else if self.aliens.iter().all(|alien| !alien.alive) {
            self.wave_banner_frames = self.banner_duration;
        }
    }

    pub fn banner(&self) -> Option<&'static str> {
        (self.wave_banner_frames > 0).then_some(WAVE_CLEARED_BANNER)
    }

    fn spawn_wave(&mut self) {
        self.aliens = spawn_grid(&self.config);
        self.bullets.clear();
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
                    break;
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
            GameInput::Fire if self.bullets.is_empty() => {
                self.bullets.push(Bullet::new(
                    self.player.x,
                    self.player.y - 1.0,
                    self.config.bullet_symbol,
                    -self.config.bullet_speed,
                ));
            }
            _ => {}
        }

        self.player.x = self
            .player
            .x
            .clamp(0.0, self.config.window_width as f32 - 1.0);
    }
}

fn spawn_grid(config: &GameConfig) -> Vec<Alien> {
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
    aliens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fire_spawns_at_most_one_bullet() {
        let mut state = GameState::new(GameConfig::new());

        state.handle_input(GameInput::Fire);
        state.handle_input(GameInput::Fire);

        assert_eq!(state.bullets.len(), 1);
    }

    #[test]
    fn bullet_destroying_alien_scores_ten() {
        let mut state = GameState::new(GameConfig::new());
        state.player.x = state.aliens[0].x;

        state.handle_input(GameInput::Fire);
        for _ in 0..30 {
            state.update();
        }

        assert_eq!(state.aliens.iter().filter(|alien| !alien.alive).count(), 1);
        assert_eq!(state.score, 10);
    }

    #[test]
    fn clearing_wave_shows_banner_then_respawns() {
        let mut state = GameState::new(GameConfig::new());
        let wave_size = state.aliens.len();

        for alien in &mut state.aliens {
            alien.alive = false;
        }

        state.update();
        assert_eq!(state.banner(), Some("WAVE CLEARED"));

        for _ in 0..200 {
            state.update();
        }

        assert_eq!(state.banner(), None);
        assert_eq!(state.aliens.len(), wave_size);
        assert!(state.aliens.iter().all(|alien| alien.alive));
    }
}
