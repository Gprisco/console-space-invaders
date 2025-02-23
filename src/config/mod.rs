#[derive(Debug, Clone)]
pub struct GameConfig {
    // Window configuration
    pub window_width: u16,
    pub window_height: u16,

    // Player configuration
    pub player_speed: f32,
    pub player_symbol: char,

    // Alien configuration
    pub alien_rows: u16,
    pub alien_columns: u16,
    pub alien_symbol: char,
    pub alien_speed: f32,

    // Bullet configuration
    pub bullet_symbol: char,
    pub bullet_speed: f32,

    // Game settings
    pub fps: u32,
    pub initial_lives: u8,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 80,
            window_height: 24,
            player_speed: 1.0,
            player_symbol: '▀',
            alien_rows: 3,
            alien_columns: 8,
            alien_symbol: '👾',
            alien_speed: 0.5,
            bullet_symbol: '|',
            bullet_speed: 1.0,
            fps: 60,
            initial_lives: 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl GameConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_difficulty(&mut self, difficulty: Difficulty) -> &mut Self {
        match difficulty {
            Difficulty::Easy => {
                self.alien_speed = 0.3;
                self.initial_lives = 5;
            }
            Difficulty::Normal => {
                // Default values
            }
            Difficulty::Hard => {
                self.alien_speed = 0.8;
                self.initial_lives = 2;
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GameConfig::default();
        assert_eq!(config.window_width, 80);
        assert_eq!(config.window_height, 24);
        assert_eq!(config.initial_lives, 3);
    }

    #[test]
    fn test_difficulty_settings() {
        let mut config = GameConfig::new();

        // Test Easy difficulty
        config.with_difficulty(Difficulty::Easy);
        assert_eq!(config.initial_lives, 5);
        assert_eq!(config.alien_speed, 0.3);

        // Test Hard difficulty
        config.with_difficulty(Difficulty::Hard);
        assert_eq!(config.initial_lives, 2);
        assert_eq!(config.alien_speed, 0.8);
    }
}
