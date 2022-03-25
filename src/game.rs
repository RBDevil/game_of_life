use rand::Rng;
use std::thread;
use std::time::Duration;

struct Coordinate {
    x: usize,
    y: usize,
}

pub struct Game {
    board: Vec<Vec<bool>>,
}

impl Game {
    pub fn new(size: usize, fill_percentage: u8, living_edge: bool) -> Game {
        let mut v1: Vec<Vec<bool>> = Vec::new();

        // create with false values
        for i in 0..size + 2 {
            v1.push(Vec::new());
            for j in 0..size + 2 {
                v1[i].push(false);
            }
        }

        // set edges to true
        if living_edge {

            for i in 0..size + 1 {
                v1[0][i] = true;
                v1[size + 1][i] = true;
            }
            for i in 0..size + 1 {
                v1[i][0] = true;
                v1[i][size + 1] = true;
            }
        }

        let mut game = Game { board: v1 };
        game.fill_with_randoms(fill_percentage);
        game
    }

    pub fn start(mut self) {

        loop {

            self.show();
            self.update();
            thread::sleep(Duration::new(1,0));
        }
    }

    pub fn insert_pattern(&mut self, x: usize, y: usize, pattern: Vec<Vec<bool>>) {
        for i in 0..pattern.len() {
            for j in 0..pattern[i].len() {
                self.board[i + x][j + y] = pattern[i][j];
            }
        }
    }

    fn update(&mut self) {
        self.flip_cells(self.get_cells_to_flip());
    }

    fn flip_cells(&mut self, cells_to_flip: Vec<Coordinate>) {

        for coordinate in cells_to_flip.iter() {
            self.board[coordinate.x][coordinate.y] =
                !self.board[coordinate.x][coordinate.y];
        }
    }

    fn get_cells_to_flip(&self) -> Vec<Coordinate> {

        let mut cells_to_flip: Vec<Coordinate> = Vec::new();

        for i in 1..self.board.len() - 1 {
            for j in 1..self.board[i].len() - 1 {

                let coordinate = Coordinate { x: i, y: j };
                let neighbour_count = self.count_neighbours(&coordinate);
                if !self.board[i][j] {
                    if neighbour_count == 3 {
                        cells_to_flip.push(coordinate);
                    }
                }
                else if neighbour_count > 3 || neighbour_count < 2 {
                    cells_to_flip.push(coordinate);
                }
            }
        }

        cells_to_flip
    }

    fn fill_with_randoms(&mut self, chance: u8) {

        let mut rng = rand::thread_rng();

        for i in 1..self.board.len() - 1 {
            for j in 1..self.board.len() - 1 {
                let num = rng.gen_range(0..100); {
                    if num < chance {
                        self.board[i][j] = true;
                    }
                }
            }
        }
    }

    fn show(&self) {

        println!();
        for i in 1..self.board.len() - 1 {
            for j in 1..self.board[i].len() - 1 {
                if self.board[i][j] {
                    std::print!("  @")
                }
                else { std::print!("  -") }
            }
            std::println!();
        }
    }

    fn count_neighbours(&self, coordinate: &Coordinate) -> i32 {

        let mut count = 0;
        for i in 0..3 {
            for j in 0..3 {
                if !(i == 1 && j == 1) && self.board[coordinate.x + i - 1][coordinate.y + j - 1] {
                    count = count + 1;
                }
            }
        }
        count
    }
}