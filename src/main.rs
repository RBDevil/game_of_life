use crate::game::Game;

mod game;

fn main() {
    let mut game = Game::new(15, 0, true);
    let mut pattern: Vec<Vec<bool>> = Vec::new();
    for i in 0..3 {
        pattern.push(Vec::new());
        pattern[i].push(true);
    }
    game.insert_pattern(5, 5, pattern);
    game.start();
}
