use crate::ai_manager;
use crate::board::State::{Empty, X, O};

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum State {
    X,
    O,
    Empty
}

#[derive(Clone)]
pub struct Board {
    pub(crate) board_size: usize,
    pub(crate) cells: Vec<State>,
}

impl Board {
    pub fn show(&self) {
        let mut delimiter = false;
        if self.board_size > 3 {
            delimiter = true;
        }
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                match self.cells[i * self.board_size + j] {
                    State::X => {
                        print!("X  ");
                    }
                    State::O => {
                        print!("O  ");
                    }
                    State::Empty => {
                        print!("I  ");
                    }
                }
            }
            print!("\n");
        }

        print!("\nCoordinates mapping:\n");

        let mut iter = 1;
        for _ in 0..self.board_size {
            for _ in 0..self.board_size {
                if delimiter && iter > 9 {
                    print!("{} ", iter);
                } else {
                    print!("{}  ", iter);
                }
                iter += 1;
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn is_valid(&self, number: usize) -> bool {
        return self.cells[number - 1] == Empty;
    }

    pub fn update(&mut self, cell_number: usize, player: State) {
        self.cells[cell_number] = player;
    }

    pub fn clear(& mut self) {
        let limit = self.cells.len();
        for i in 0..limit{
            self.cells[i] = Empty;
        }
    }
}

pub fn check_end(cells: Vec<State>) -> (bool, State) {
    if check_end_inner(cells.clone(), X) {
        return (true, X);
    }
    if check_end_inner(cells.clone(), O) {
        return (true, O);
    }
    return (false, Empty);
}

pub fn check_end_inner(cells: Vec<State>, player: State) -> bool {
    let size = f32::sqrt(cells.len() as f32) as usize;
    let win_con = 3 + (((size - 3) / 2) as f32).floor() as usize;
    let mut best = 0;
    let mut temp = 0;

    //vertical lines
    for i in 0..size {
        for j in 0..size {
            if cells[i * size + j] == player { temp += 1 } else { temp = 0 };
            if temp == win_con { return true}
        }
        best = best.max(temp);
        temp = 0;
    }

    //horizontal lines
    for i in 0..size {
        for j in 0..size {
            if cells[j * size + i] == player { temp += 1 } else { temp = 0 };
            if temp == win_con { return true}
        }
        best = best.max(temp);
        temp = 0;
    }

    let oblique_limit = size - win_con + 1 as usize;

    //oblique lines :decreasing /lower-half
    for i in 0..oblique_limit {
        for j in 0..(size - i) {
            if cells[(i + j) * size + j] == player { temp += 1 } else { temp = 0 };
            if temp == win_con { return true}
        }
        best = best.max(temp);
        temp = 0;
    }

    //oblique lines :decreasing /upper-half
    for i in 0..oblique_limit {
        for j in 0..(size - i) {
            if cells[i + j * size + j] == player { temp += 1 } else { temp = 0 };
            if temp == win_con { return true}
        }
        best = best.max(temp);
        temp = 0;
    }


    //oblique lines :increasing /lower-half
    for i in 0..oblique_limit {
        for j in 0..(size - i) {
            if cells[(size - 1) + i * (size) + j * (size - 1)] == player { temp += 1 } else { temp = 0 };
            if temp == win_con { return true}
        }
        best = best.max(temp);
        temp = 0;
    }

    //oblique lines :increasing /upper-half
    for i in win_con..size {
        for j in 0..(i) {
            if cells[i + j * (size - 1) - 1] == player { temp += 1 } else { temp = 0 };
            if temp == win_con { return true}
        }
        best = best.max(temp);
        temp = 0;
    }

    return if best == win_con {
        true
    } else {
        false
    };
}



