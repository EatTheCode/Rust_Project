#[derive(Clone, Copy, Debug)]
enum Cell {
    Hidden,
    Revealed,
    Flagged,
    Mine,
}

// Debug 트레잇을 derive하여 `Board`를 디버깅 가능하게 만듭니다.
#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(width: usize, height: usize, mines: usize) -> Board {
        let mut board = Board {
            width,
            height,
            cells: vec![vec![Cell::Hidden; width]; height],
        };

        // 지뢰 심기 로직
        for _ in 0..mines {
            let x = rand::random::<usize>() % width;
            let y = rand::random::<usize>() % height;
            board.cells[y][x] = Cell::Mine;
        }

        board
    }
}

fn main() {
    let board = Board::new(10, 10, 10);
    println!("{:?}", board);
}
