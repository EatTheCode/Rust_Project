use std::io;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Hidden { adjacent_mines: usize },
    Revealed { adjacent_mines: usize },
    Flagged,
    Mine,
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    game_over: bool,
}

impl Board {
    fn new(width: usize, height: usize, mines: usize) -> Board {
        let mut board = Board {
            width,
            height,
            cells: vec![vec![Cell::Hidden { adjacent_mines: 0 }; width]; height],
            game_over: false,
        };

        // 지뢰 심기
        let mut rng = rand::thread_rng();
        let mut placed_mines = 0;
        while placed_mines < mines {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if matches!(board.cells[y][x], Cell::Hidden { adjacent_mines: 0 }) {
                board.cells[y][x] = Cell::Mine;
                placed_mines += 1;
            }
        }

        // 주변 지뢰 수 계산
        for y in 0..height {
            for x in 0..width {
                if matches!(board.cells[y][x], Cell::Mine) {
                    continue;
                }
                let adjacent_mines = board.count_adjacent_mines(x, y);
                board.cells[y][x] = Cell::Hidden { adjacent_mines };
            }
        }

        board
    }

    fn count_adjacent_mines(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in y.saturating_sub(1)..=y + 1 {
            for j in x.saturating_sub(1)..=x + 1 {
                if i >= self.height || j >= self.width || (i == y && j == x) {
                    continue;
                }
                if matches!(self.cells[i][j], Cell::Mine) {
                    count += 1;
                }
            }
        }
        count
    }

    fn reveal(&mut self, x: usize, y: usize) {
        if self.game_over {
            return;
        }

        match self.cells[y][x] {
            Cell::Hidden { adjacent_mines } => {
                self.cells[y][x] = Cell::Revealed { adjacent_mines };
                if adjacent_mines == 0 {
                    for i in y.saturating_sub(1)..=y + 1 {
                        for j in x.saturating_sub(1)..=x + 1 {
                            if i >= self.height || j >= self.width || (i == y && j == x) {
                                continue;
                            }
                            if let Cell::Hidden { .. } = self.cells[i][j] {
                                self.reveal(j, i);
                            }
                        }
                    }
                }
            },
            Cell::Mine => {
                self.game_over = true;
                println!("Boom! You hit a mine.");
            },
            _ => {}
        }
    }

    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.cells[y][x] {
                    Cell::Hidden { .. } => print!("□ "),
                    Cell::Revealed { adjacent_mines } if adjacent_mines > 0 => print!("{} ", adjacent_mines),
                    Cell::Revealed { .. } => print!("  "),
                    Cell::Flagged => print!("⚑ "),
                    Cell::Mine if self.game_over => print!("* "),
                    Cell::Mine => print!("□ "),
                }
            }
            println!();
        }
    }

    fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.game_over {
            return;
        }

        match self.cells[y][x] {
            Cell::Hidden { adjacent_mines: _ } => {
                self.cells[y][x] = Cell::Flagged;
            },
            Cell::Flagged => {
                self.cells[y][x] = Cell::Hidden { adjacent_mines: 0 }; // 임시로 0을 넣음, 실제로는 이전 값을 유지해야 함
            },
            _ => {}
        }
    }

    fn play(&mut self) {
        while !self.game_over {
            self.display();
            let mut input = String::new();
            println!("Enter coordinates to reveal or flag (e.g., r 3,2 or f 3,2 for reveal or flag):");
            io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();

            if parts.len() == 2 {
                let cmd = parts[0];
                let coords: Vec<&str> = parts[1].split(',').collect();
                if coords.len() == 2 {
                    if let (Ok(x), Ok(y)) = (coords[0].parse::<usize>(), coords[1].parse::<usize>()) {
                        if x < self.width && y < self.height {
                            match cmd {
                                "r" => self.reveal(x, y),
                                "f" => self.toggle_flag(x, y),
                                _ => println!("Unknown command! Use 'r' to reveal or 'f' to flag."),
                            }
                        } else {
                            println!("Coordinates are out of bounds!");
                        }
                    } else {
                        println!("Invalid coordinates!");
                    }
                } else {
                    println!("Please enter a command followed by coordinates in the format cmd x,y!");
                }
            } else {
                println!("Please enter a command followed by coordinates in the format cmd x,y!");
            }
        }
    }
}

fn main() {
    let mut board = Board::new(10, 10, 10);
    board.play();
}
