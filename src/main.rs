mod config;

fn main() {
    let config = config::GameConfig::new();
    println!("Game configuration loaded: {:?}", config);
}
