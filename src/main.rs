use std::fmt;
use rand::Rng;
use std::{thread, time};

const BOARD_X: i32 = 64;
const BOARD_Y: i32 = 20;

#[derive(PartialEq, Copy, Clone)]
enum CellState {
    Alive,
    Dead
}

enum Pattern {
    Glider,
    Random
}

struct Board {
    state: [[CellState; BOARD_X as usize]; BOARD_Y as usize],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        Ok(for row in self.state.iter() {
            write!(f, "| ")?;
            for cell in row.iter() {
                let cell_char = match cell {
                    CellState::Dead  => '.',
                    CellState::Alive => '#',
                };
                write!(f, "{}", cell_char)?;
            }
            writeln!(f, " |")?;
        })
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: [[CellState::Dead; BOARD_X as usize]; BOARD_Y as usize]
        }
    }

    fn get_next_cell_state(&self, point: (i32, i32)) -> CellState {
        let live_neighbours = self.count_live_neighbours(point);

        match self.get_cell_state(point) {
            CellState::Alive => {
                if live_neighbours < 2 || live_neighbours > 3 {
                    CellState::Dead
                } else {
                    CellState::Alive
                }
            }
            CellState::Dead => {
                if live_neighbours == 3 {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            }
        }
    }

    fn count_live_neighbours(&self, (x, y): (i32, i32)) -> i32 {
        let mut alive_count = 0;
        for local_y in y-1..y+2 {
            for local_x in x-1..x+2 {
                    if !(local_x == x && local_y == y) && self.get_cell_state((local_x, local_y)) == CellState::Alive {
                        alive_count += 1;
                    }
            }
        }
        alive_count
    }

    fn get_cell_state(&self, (x, y): (i32, i32)) -> CellState {
        let (wrapped_x, wrapped_y) = (x.rem_euclid(BOARD_X), y.rem_euclid(BOARD_Y));
        self.state[wrapped_y as usize][wrapped_x as usize]
    }

    fn set_cell_state_with_offset(&mut self, (x, y): (i32, i32), state: CellState, (x_offset, y_offset): (i32, i32)) {
        let offset_coords = (x+x_offset, y+y_offset);
        self.set_cell_state(offset_coords, state);
    }

    fn set_cell_state(&mut self, (x, y): (i32, i32), state: CellState) {
        let (wrapped_x, wrapped_y) = (x.rem_euclid(BOARD_X), y.rem_euclid(BOARD_Y));
        self.state[wrapped_y as usize][wrapped_x as usize] = state;
    }
}

fn get_next_board(board: &Board) -> Board {
    let mut next_board = Board::new();
    for (y, row) in board.state.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            next_board.state[y][x] = board.get_next_cell_state((x as i32, y as i32));
        }
    }
    next_board
}

fn insert_pattern(board: &mut Board, pattern: Pattern, offset: (i32, i32)) {
    match pattern {
        Pattern::Glider => {
            board.set_cell_state_with_offset((0, 0), CellState::Alive, offset);
            board.set_cell_state_with_offset((0, 2), CellState::Alive, offset);
            board.set_cell_state_with_offset((1, 1), CellState::Alive, offset);
            board.set_cell_state_with_offset((1, 2), CellState::Alive, offset);
            board.set_cell_state_with_offset((2, 1), CellState::Alive, offset);
        }
        Pattern::Random => {
            let mut rng = rand::thread_rng();
            for y in 0..board.state.len() as i32 {
                for x in 0..board.state.len() as i32 {
                    let r: i32 = rng.gen();
                    if r % 3 == 0 {
                        board.set_cell_state((x, y), CellState::Alive);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    
    insert_pattern(&mut board, Pattern::Glider, (0, 0));
    insert_pattern(&mut board, Pattern::Glider, (16, 0));
    insert_pattern(&mut board, Pattern::Glider, (32, 0));
    insert_pattern(&mut board, Pattern::Glider, (48, 0));

    let sleep_time = time::Duration::from_millis(50);
    let mut iteration_num = 0;
    loop {
        println!(".__________________________ Iteration {} __________________________.", iteration_num);
        println!("{}", board);
        board = get_next_board(&board);
        thread::sleep(sleep_time);
        iteration_num += 1;
    }
}
