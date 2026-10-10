use super::alien::Alien;
use super::bullet::Bullet;
use super::player::Player;
use crate::config::GameConfig;
use crate::input::GameInput;

const WAVE_CLEARED_BANNER: &str = "WAVE CLEARED";
const GAME_OVER_BANNER: &str = "GAME OVER";
const ALIEN_DROP_AMOUNT: f32 = 1.0;

pub enum GameStatus {
    Playing,
    #[allow(dead_code)]
    Paused,
    GameOver,
}

pub struct GameState {
    pub player: Player,
    pub aliens: Vec<Alien>,
    pub bullets: Vec<Bullet>,
    pub score: u32,
    pub lives: u8,
    pub status: GameStatus,
    config: GameConfig,
    wave_banner_frames: u32,
    banner_duration: u32,
    alien_fire_timer: u32,
    fire_seed: u64,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        let player = Player::new(
            f32::from(config.window_width) / 2.0,
            f32::from(config.window_height) - 2.0,
            config.player_symbol,
        );

        Self {
            player,
            aliens: spawn_grid(&config),
            bullets: Vec::new(),
            score: 0,
            lives: config.initial_lives,
            status: GameStatus::Playing,
            banner_duration: config.fps.saturating_mul(3) / 2,
            config,
            wave_banner_frames: 0,
            alien_fire_timer: 0,
            fire_seed: 0x9E37_79B9_7F4A_7C15,
        }
    }

    pub fn update(&mut self) {
        if matches!(self.status, GameStatus::GameOver) {
            return;
        }

        self.update_aliens();
        self.update_bullets();
        self.check_collisions();

        if self.invasion_reached_player() {
            self.status = GameStatus::GameOver;
            return;
        }

        self.maybe_alien_fire();

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
        if matches!(self.status, GameStatus::GameOver) {
            return Some(GAME_OVER_BANNER);
        }
        (self.wave_banner_frames > 0).then_some(WAVE_CLEARED_BANNER)
    }

    fn spawn_wave(&mut self) {
        self.aliens = spawn_grid(&self.config);
        self.bullets.clear();
        self.alien_fire_timer = 0;
    }

    fn update_aliens(&mut self) {
        for alien in &mut self.aliens {
            if alien.alive {
                alien.update(self.config.alien_speed);
            }
        }

        let width = f32::from(self.config.window_width);
        let hit_edge = self
            .aliens
            .iter()
            .filter(|alien| alien.alive)
            .any(|alien| alien.x <= 0.0 || alien.x >= width - 2.0);
        if hit_edge {
            for alien in &mut self.aliens {
                if alien.alive {
                    alien.reverse_direction(ALIEN_DROP_AMOUNT);
                }
            }
        }
    }

    fn update_bullets(&mut self) {
        for bullet in &mut self.bullets {
            bullet.update();
            if bullet.is_out_of_bounds(self.config.window_height) {
                bullet.active = false;
            }
        }
        self.bullets.retain(|bullet| bullet.active);
    }

    fn maybe_alien_fire(&mut self) {
        self.alien_fire_timer = self.alien_fire_timer.saturating_add(1);
        if self.alien_fire_timer < self.config.alien_fire_interval {
            return;
        }
        self.alien_fire_timer = 0;

        let live_alien_bullets = self
            .bullets
            .iter()
            .filter(|bullet| bullet.active && bullet.is_alien())
            .count();
        if live_alien_bullets >= self.config.max_alien_bullets {
            return;
        }

        let Some((x, y)) = self.pick_shooter() else {
            return;
        };
        self.bullets.push(Bullet::new(
            x,
            y + 1.0,
            self.config.alien_bullet_symbol,
            self.config.bullet_speed,
        ));
    }

    fn pick_shooter(&mut self) -> Option<(f32, f32)> {
        let mut columns: Vec<(i32, f32, f32)> = Vec::new();
        for alien in &self.aliens {
            if !alien.alive {
                continue;
            }
            let column = alien.x.round() as i32;
            match columns.iter_mut().find(|(x, _, _)| *x == column) {
                Some(entry) => {
                    if alien.y > entry.2 {
                        entry.1 = alien.x;
                        entry.2 = alien.y;
                    }
                }
                None => columns.push((column, alien.x, alien.y)),
            }
        }
        if columns.is_empty() {
            return None;
        }
        self.fire_seed = self
            .fire_seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        let index = (self.fire_seed >> 33) as usize % columns.len();
        Some((columns[index].1, columns[index].2))
    }

    fn invasion_reached_player(&self) -> bool {
        self.aliens
            .iter()
            .any(|alien| alien.alive && alien.y >= self.player.y)
    }

    fn check_collisions(&mut self) {
        for bullet in &mut self.bullets {
            if !bullet.active || bullet.is_alien() {
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

        let mut player_hit = false;
        for bullet in &mut self.bullets {
            if !bullet.active || !bullet.is_alien() {
                continue;
            }
            if (bullet.x - self.player.x).abs() < 1.0 && (bullet.y - self.player.y).abs() < 1.0 {
                bullet.active = false;
                player_hit = true;
                break;
            }
        }

        if player_hit {
            self.lives = self.lives.saturating_sub(1);
            if self.lives == 0 {
                self.status = GameStatus::GameOver;
            } else {
                for bullet in &mut self.bullets {
                    if bullet.is_alien() {
                        bullet.active = false;
                    }
                }
            }
        }
    }

    pub fn handle_input(&mut self, input: GameInput) {
        if matches!(self.status, GameStatus::GameOver) {
            return;
        }
        match input {
            GameInput::Left => {
                self.player.move_left(self.config.player_speed);
            }
            GameInput::Right => {
                self.player.move_right(self.config.player_speed);
            }
            GameInput::Fire
                if !self
                    .bullets
                    .iter()
                    .any(|bullet| bullet.active && !bullet.is_alien()) =>
            {
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
            .clamp(0.0, f32::from(self.config.window_width) - 1.0);
    }
}

fn spawn_grid(config: &GameConfig) -> Vec<Alien> {
    let mut aliens = Vec::new();
    for row in 0..config.alien_rows {
        for col in 0..config.alien_columns {
            aliens.push(Alien::new(
                f32::from(col * 2 + 2),
                f32::from(row * 2 + 1),
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
        let mut config = GameConfig::new();
        config.alien_speed = 0.0;
        config.alien_fire_interval = u32::MAX;
        let mut state = GameState::new(config);
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

    #[test]
    fn aliens_advance_sideways_each_tick() {
        let mut config = GameConfig::new();
        config.alien_fire_interval = u32::MAX;
        let mut state = GameState::new(config);
        let before: Vec<f32> = state.aliens.iter().map(|alien| alien.x).collect();

        state.update();

        for (alien, x) in state.aliens.iter().zip(before) {
            assert_eq!(alien.x, x + 0.5);
        }
    }

    #[test]
    fn aliens_reverse_and_drop_at_playfield_edge() {
        let mut config = GameConfig::new();
        config.alien_fire_interval = u32::MAX;
        let mut state = GameState::new(config);
        let edge = f32::from(state.config.window_width) - 2.0;
        for alien in &mut state.aliens {
            alien.x = edge;
        }
        let before_y: Vec<f32> = state.aliens.iter().map(|alien| alien.y).collect();

        state.update();

        for (alien, y) in state.aliens.iter().zip(before_y) {
            assert_eq!(alien.direction, -1.0);
            assert_eq!(alien.y, y + 1.0);
        }
    }

    #[test]
    fn alien_fire_spawns_downward_bullet_on_interval() {
        let mut config = GameConfig::new();
        config.alien_speed = 0.0;
        config.alien_fire_interval = 1;
        config.max_alien_bullets = 3;
        let mut state = GameState::new(config);

        state.update();

        assert_eq!(state.bullets.len(), 1);
        assert!(state.bullets[0].is_alien());
    }

    #[test]
    fn alien_fire_respects_max_concurrent_bullets() {
        let mut config = GameConfig::new();
        config.alien_speed = 0.0;
        config.alien_fire_interval = 1;
        config.max_alien_bullets = 2;
        let mut state = GameState::new(config);

        for _ in 0..20 {
            state.update();
        }

        let alien_bullets = state
            .bullets
            .iter()
            .filter(|bullet| bullet.is_alien())
            .count();
        assert!(alien_bullets <= 2);
    }

    #[test]
    fn player_can_fire_while_alien_bullet_present() {
        let mut config = GameConfig::new();
        config.alien_fire_interval = u32::MAX;
        let mut state = GameState::new(config);
        state.bullets.push(Bullet::new(5.0, 5.0, '!', 1.0));

        state.handle_input(GameInput::Fire);
        state.handle_input(GameInput::Fire);

        let player_bullets = state
            .bullets
            .iter()
            .filter(|bullet| !bullet.is_alien())
            .count();
        assert_eq!(player_bullets, 1);
        assert_eq!(state.bullets.len(), 2);
    }

    #[test]
    fn alien_bullet_hit_costs_one_life_and_clears_volley() {
        let mut config = GameConfig::new();
        config.alien_speed = 0.0;
        config.alien_fire_interval = u32::MAX;
        let mut state = GameState::new(config);
        let lives = state.lives;
        state
            .bullets
            .push(Bullet::new(state.player.x, state.player.y - 1.0, '!', 1.0));
        state.bullets.push(Bullet::new(2.0, 2.0, '!', 1.0));

        state.update();

        assert_eq!(state.lives, lives - 1);
        assert_eq!(
            state
                .bullets
                .iter()
                .filter(|bullet| bullet.active && bullet.is_alien())
                .count(),
            0
        );
    }

    #[test]
    fn lethal_hit_ends_game_with_banner_and_freeze() {
        let mut config = GameConfig::new();
        config.alien_speed = 0.5;
        config.alien_fire_interval = u32::MAX;
        let mut state = GameState::new(config);
        state.lives = 1;
        state
            .bullets
            .push(Bullet::new(state.player.x, state.player.y - 1.0, '!', 1.0));

        state.update();

        assert_eq!(state.lives, 0);
        assert_eq!(state.banner(), Some("GAME OVER"));

        let alien_x: Vec<f32> = state.aliens.iter().map(|alien| alien.x).collect();
        state.update();
        let frozen_x: Vec<f32> = state.aliens.iter().map(|alien| alien.x).collect();
        assert_eq!(alien_x, frozen_x);
    }

    #[test]
    fn invasion_at_player_row_ends_game() {
        let mut config = GameConfig::new();
        config.alien_fire_interval = u32::MAX;
        let mut state = GameState::new(config);
        state.aliens[0].y = state.player.y;

        state.update();

        assert_eq!(state.banner(), Some("GAME OVER"));
    }
}
