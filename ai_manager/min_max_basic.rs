use std::cmp::max;
use crate::{ai_manager, board};
use crate::board::State;
use crate::board::State::{Empty, X, O};

pub fn run(cells: Vec<State>, depth: i8, mut player: State) -> usize {
    let mut value = i32::MIN;
    let size = cells.len();
    let mut next_cells: Vec<State> = cells.clone();
    let mut temp;
    let mut best_move = 0;
    let mut maximize = true;
    if player == O {
        value = i32::MAX;
        maximize = false;
    }
    // for each child of node do
    //go over all possibilities
    for i in 0..size {
        if next_cells[i] == Empty {
            next_cells[i] = player;
            temp = run_recursive(next_cells.clone(), depth - 1, !maximize);
            println!("{i} move for X is worth: {temp}");
            if maximize {
                if temp >= value {
                    best_move = i;
                    value = temp;
                }
            } else {
                if temp <= value {
                    best_move = i;
                    value = temp;
                }
            }
            next_cells[i] = Empty;
        }
    }
    println!("\nComputer chosen {} with value {value}\n\n", best_move);
    return best_move;
}

pub fn run_recursive(cells: Vec<State>, depth: i8, maximize: bool) -> i32 {
    let mut player = X;
    let mut next_player = O;
    if !maximize {
        player = O;
        next_player = X;
    }
    let (boolean, winner) = board::check_end(cells.clone());
    if boolean { return if winner == X { i32::MAX } else { i32::MIN }; };
    if depth == 0 {
        return ai_manager::cost_function_0(cells.clone(), next_player);
    }

    let size = cells.len();
    return if maximize {
        let mut value = i32::MIN;

        let mut next_cells: Vec<State> = cells.clone();
        // for each child of node do
        //go over all possibilities
        for i in 0..size {
            if next_cells[i] == Empty {
                next_cells[i] = player;
                value = value.max(run_recursive(next_cells.clone(), depth - 1, !maximize));
                next_cells[i] = Empty;
            }
        }
        let (boolean, winner) = board::check_end(cells.clone());
        if boolean { return if winner == X { i32::MAX } else { i32::MIN }; }
        else if value == i32::MIN {
            return 0;
        }
        return value;
        // return value
    } else {
        let mut value = i32::MAX;
        let mut next_cells: Vec<State> = cells.clone();
        // for each child of node do
        //go over all possibilities
        for i in 0..size {
            if next_cells[i] == Empty {
                next_cells[i] = player;
                value = value.min(run_recursive(next_cells.clone(), depth - 1, !maximize));
                next_cells[i] = Empty;
            }
        }
        let (boolean, winner) = board::check_end(cells.clone());
        if boolean { return if winner == X { i32::MAX } else { i32::MIN }; }
        else if value == i32::MAX {
            return 0;
        }
        return value;
    };
}


// else (* minimizing player *)
// value := +∞
// for each child of node do
// value := min(value, minimax(child, depth − 1, TRUE))
