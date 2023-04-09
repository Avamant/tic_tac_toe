use std::io::{stdout, stdin, Write};
use board::Board;
use crate::board;

pub fn get_board_size() -> usize {
    println!("\nEnter board size 3-10 (default = 3): ");
    let mut got_correct_input: bool = false;
    let mut input_string: String = String::new();
    let mut input_board_size = 3;
    while !got_correct_input {
        stdin().read_line(&mut input_string).expect("Couldn't read from stdin.");
        input_string.pop(); //remove '\n' from stdin
        match input_string.parse::<usize>() {
            Ok(number) => {
                if number > 10 || number < 3 {
                    println!("{} is not correct board size.\n\
            It should be a number between 3-10 (default = 3).", number);
                    input_string = String::new();
                } else {
                    got_correct_input = true;
                    input_board_size = number;
                }
            }
            Err(err) => println!("{} is not correct board size.\n\
            It should be a number between 3-10 (default = 3).", err),
        }
        input_string = String::new();
    }
    print!("You've chosen {}", input_board_size);
    return input_board_size;
}

pub fn get_user_move(board: Board) -> usize {
    println!("\nWhat's your move? ");
    let mut got_correct_input: bool = false;
    let mut input_string: String = String::new();
    let mut input_board_size = 3;
    while !got_correct_input {
        stdin().read_line(&mut input_string).expect("Couldn't read from stdin.");
        input_string.pop();
        match input_string.parse::<usize>() {
            Ok(number) => {
                if number > 0 && number <= board.board_size * board.board_size &&
                    board.is_valid(number as usize) {
                    got_correct_input = true;
                    input_board_size = number;
                } else {
                    println!("{} is not correct input.
                    \nTry another number", number);
                }
            }
            Err(err) => println!("{} is not correct cell.\n {}", input_string, err),
        }
        input_string = String::new();
    }
    print!("You've chosen {} \n", input_board_size);
    return input_board_size;
}

pub fn get_next_game() -> bool {
    println!("\nFancy another go (Y/n)? ");
    let mut got_correct_input: bool = false;
    let mut input_string: String = String::new();
    while !got_correct_input {
        stdin().read_line(&mut input_string).expect("Couldn't read from stdin.");
        input_string.pop();
        if input_string == "Y" || input_string == "y" {
            return true;
        } else if input_string == "N" || input_string == "n"{
            return false;
        }else{
            println!("{} is not correct answer.\n", input_string);
        }
        input_string = String::new();
    }
    return false;
}