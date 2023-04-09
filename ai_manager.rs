use std::cmp::max;
use crate::board;
use crate::board::{Board, State};
use crate::board::State::Empty;

mod min_max_basic;
mod min_max_def;


pub struct AiManager {
    pub ai_model: &'static str,
    pub depth: i8,
}

impl AiManager {
    pub fn get_next_move(&self, board: Board, player: State) -> usize {
        return match self.ai_model {
            "min_max_basic" => {
                min_max_basic::run(board.cells, self.depth, player)
            }
            "min_max_def" => {
                min_max_def::run(board.cells, self.depth, player)
            }
            _ => {
                0
            }
        } as usize;
    }
}

pub fn cost_function_1(cells: Vec<board::State>, player : State) -> i32 {

    return if player == State::X {
        cost_function_inner_0(cells.clone(), player, true)
    }else{
        cost_function_inner_0(cells.clone(), player, false)
    }

}

pub fn cost_function_0(cells: Vec<board::State>, next_player : State) -> i32 {
    let t0 = cost_function_inner_0(cells.clone(), State::X, true);
    let t1 = cost_function_inner_0(cells.clone(), State::O, false);
    return if next_player == State::X {
        t0  + (0.5 * t1 as f32) as i32
    } else {
        (0.5 * t0 as f32) as i32 + t1
    }
    //println!("{t0} for X, and {t1} for O");

}

pub fn cost_function_inner_0(cells: Vec<board::State>, player: State, maximize: bool) -> i32 {
    let size = f32::sqrt(cells.len() as f32) as usize;
    let win_con = 3 + (((size - 3) / 2) as f32).floor() as usize;
    let mut best = 1;
    let mut best2 = 1;
    let mut temp = size;
    let mut state;
    let mut streak = 0;
    let mut exists = false;

    //vertical lines
    for i in 0..size {
        for j in 0..size {
            state = cells[i * size + j];
            if state == Empty {
                streak += 1;
            } else if state == player {
                streak += 1;
                temp *= 5;
            } else {
                streak = 0;
            }
            if streak == win_con {
                exists = true;
            }else if streak == win_con +1 {
                temp*=2;
            }else if streak == win_con +2 {
                temp*=2;
            }
        }
        if !exists {
            temp = 1;
        }
        if best == temp {
            best2 = temp;
        } else {
            best = best.max(temp);
        }
        temp = 1;
        streak = 0;
        exists = false;
    }

    //horizontal lines
    for i in 0..size {
        for j in 0..size {
            state = cells[j * size + i];
            if state == Empty {
                streak += 1;
            } else if state == player {
                streak += 1;
                temp *= 5;
            } else {
                streak = 0;
            }
            if streak == win_con {
                exists = true;
            }else if streak == win_con +1 {
                temp*=2;
            }else if streak == win_con +2 {
                temp*=2;
            }
        }
        if !exists {
            temp = 1;
        }
        if best == temp {
            best2 = temp;
        } else {
            best = best.max(temp);
        }
        temp = 1;
        streak = 0;
        exists = false;
    }

    let oblique_limit = size - win_con + 1 as usize;

    let oblique_limit = size - win_con + 1 as usize;


    //oblique lines :decreasing /lower-half
    for i in 0..oblique_limit {
        for j in 0..(size - i) {
            state = cells[(i + j) * size + j];
            if state == Empty {
                streak += 1;
            } else if state == player {
                streak += 1;
                temp *= 5;
            } else {
                streak = 0;
            }
            if streak == win_con {
                exists = true;
            }else if streak == win_con +1 {
                temp*=2;
            }else if streak == win_con +2 {
                temp*=2;
            }
        }
        if !exists {
            temp = 1;
        }
        if best == temp {
            best2 = temp;
        } else {
            best = best.max(temp);
        }
        temp = size;
        streak = 0;
        exists = false;
    }

    //oblique lines :decreasing /upper-half
    for i in 0..oblique_limit {
        for j in 0..(size - i) {
            state = cells[i + j * size + j];
            if state == Empty {
                streak += 1;
            } else if state == player {
                streak += 1;
                temp *= 5;
            } else {
                streak = 0;
            }
            if streak == win_con {
                exists = true;
            }else if streak == win_con +1 {
                temp*=2;
            }else if streak == win_con +2 {
                temp*=2;
            }
        }
        if !exists {
            temp = 1;
        }
        if best == temp {
            best2 = temp;
        } else {
            best = best.max(temp);
        }
        temp = 1;
        streak = 0;
        exists = false;
    }


    //oblique lines :increasing /lower-half
    for i in 0..oblique_limit {
        for j in 0..(size - i) {
            state = cells[(size - 1) + i * (size) + j * (size - 1)];
            if state == Empty {
                streak += 1;
            } else if state == player {
                streak += 1;
                temp *= 5;
            } else {
                streak = 0;
            }
            if streak == win_con {
                exists = true;
            }else if streak == win_con +1 {
                temp*=2;
            }else if streak == win_con +2 {
                temp*=2;
            }
        }
        if !exists {
            temp = 1;
        }
        if best == temp {
            best2 = temp;
        } else {
            best = best.max(temp);
        }
        temp = 1;
        streak = 0;
        exists = false;
    }

    //oblique lines :increasing /upper-half
    for i in win_con..size {
        for j in 0..(i) {
            state = cells[i + j * (size - 1) - 1];
            if state == Empty {
                streak += 1;
            } else if state == player {
                streak += 1;
                temp *= 5;
            } else {
                streak = 0;
            }
            if streak == win_con {
                exists = true;
            }else if streak == win_con +1 {
                temp*=2;
            }else if streak == win_con +2 {
                temp*=2;
            }
        }
        if !exists {
            temp = 1;
        }
        if best == temp {
            best2 = temp;
        } else {
            best = best.max(temp);
        }
        temp = 1;
        streak = 0;
        exists = false;
    }


    if board::check_end_inner(cells.clone(), player) {
        return if maximize {
            // println!("max is {}",i32::MAX);
            i32::MAX
        } else {
            // println!("min is {}",i32::MIN);
            i32::MIN
        };
    }
    if best == best2 {
        return if maximize {
            4 * best as i32
        } else {
            -(4 * best as i32)
        };
    }
    return if maximize {
        (best as i32) * 2 + best2 as i32
    } else {
        -((best as i32) * 2 + best2 as i32)
    };

    // if best == best2 {
    //     if best == 1 {
    //         return best*2 - 1;
    //     }
    // }
    // return best*2;
}