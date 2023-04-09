mod board;
mod std_in_out;
mod ai_manager;

use crate::board::State::{X, Empty, O};
use crate::std_in_out::get_user_move;

fn main() {

    let let_user_choose_board_size = false;
    let depth = 4;

    let mut input_board_size = 3;
    if let_user_choose_board_size {
        input_board_size = std_in_out::get_board_size();
    }
    println!("\nWin condition is calculated via: (3 + floor((board_size - 3)/2)): ");
    let win_condition = 3 + ((((input_board_size - 3) / 2) as f32).floor() as i32);


    println!("Board size {}", input_board_size);
    println!("Win condition: {}", win_condition);

    let mut play_board = board::Board {
        board_size: input_board_size,
        cells: vec![board::State::Empty; (input_board_size * input_board_size) as usize],
    };

    let ai_manager = ai_manager::AiManager {
        ai_model: "min_max_def",
        depth: depth,
    };

    // main game loop
    let mut is_finished = false;
    let user = X;
    let mut move_no = 0;
    let mut next_move;
    let mut winner = Empty;


    let mut test_board = board::Board {
        board_size: 4,
        cells: vec![board::State::Empty; 16 as usize],
    };

    // I  I  I  X
    // I  I  X  I
    // I  X  I  I
    // O  O  O  I
    test_board.cells[9] = X;



    println!("{}", board::check_end_inner(test_board.cells.clone(), X));
    // return;
    test_board.show();

    println!("{}", ai_manager::cost_function_0(test_board.cells.clone(),X));

    let mut test = ai_manager.get_next_move(test_board.clone(),O);
    println!( "Chosen for test: {}",ai_manager.get_next_move(test_board.clone(),O));
    test_board.cells[test] = O;
    test_board.show();
    return;
    let limit = play_board.board_size*play_board.board_size;

    while !is_finished{
        play_board.show();

        if move_no % 2 == 0 { // X
            if user == X {
                next_move = get_user_move(play_board.clone());
                next_move -= 1;
            } else {
                next_move = ai_manager.get_next_move(play_board.clone(), X) as usize;
            }
            play_board.update(next_move, X);
        } else { // O
            if user == O {
                next_move = get_user_move(play_board.clone());
                next_move -= 1;
            } else {
                next_move = ai_manager.get_next_move(play_board.clone(), board::State::O) as usize;
            }
            play_board.update(next_move, board::State::O);
        }
        (is_finished, winner) = board::check_end(play_board.clone().cells.clone());
        move_no += 1;


        if is_finished{
            play_board.show();
            println!(" \t\tThe winner is {:?}", winner);
            is_finished = !std_in_out::get_next_game();
            play_board.clear();
            move_no = 0;
        }else if move_no == limit{
            play_board.show();
            println!(" \t\tThe match ended in a draw.");
            is_finished = !std_in_out::get_next_game();
            play_board.clear();
            move_no = 0;
        }
    }


    // if move_no % 2 == 0 {
    //     if user==X {
    //         get_user_move();
    //     }else{
    //
    //     }
    // }else {
    //     if user==X {
    //         while !is_valid() {
    //             println!(" is already occupied.
    //             \nTry another number", );
    //         }
    //         play_board.cells[get_user_move()]= X;
    //     }else{
    //
    //     }
    // }

    // play_board.check_end();
    //     O move


    // println!("start test");
    // clearscreen::clear().unwrap();
    // //std::process::Command::new("cmd").arg("/C").arg("cls");
    // println!("program started");
    //
    // cout << "Write the width of the board (3-7): ";
    // cin >> Width;
    // while (Width < 3 || Width> 7)
    // {
    //     cout << "Write the width of the board (3-7): ";
    //     cin >> Width;
    // }
    // BoardSize = Width * Width;
    //
    // cout << "How many in row to win ? (3-" << Width << "): ";
    // cin >> Condition;
    // while (Condition<3 || Condition>Width)
    // {
    //     cout << "How many in row to win ? (3-" << Width << "): ";
    //     cin >> Condition;
    // }
    // int a;
    //
    // cout << endl << "What kind of game? " << endl << endl;
    // cout << "1) Against player. " << endl;
    // cout << "2) Against computer. " << endl << endl;
    // cin >> a;
    // if (a != 1 && a != 2)
    // {
    //     do
    //     {
    //         cin >> a;
    //     } while (a != 1 && a != 2);
    //
    // }
    //
    // switch (a)
    // {
    //     case 1:
    //     {
    //         BoardManager.Player2 = 0;
    //         break;
    //     }
    //     case 2:
    //     {
    //         BoardManager.Player2 = 1;
    //         break;
    //     }
    //     break;
    // }
    //
    // BoardManager.Depth = 4;
    // vector <Point> cel;
    // cel.resize(BoardSize);
    //
    // vector <Point> allX;
    // vector <Point> allO;
    //
    // int i;
    // int x = 0;
    // int x1 = 0;
    // int y1 = 0;
    //
    // for (int b = 0; b < Width; b++)//wype?nianie zmiennych: x,y
    // {
    //     for (i = 0; i < Width; i++, x++, x1++)
    //     {
    //         cel[x].x = x1;
    //         cel[x].y = y1;
    //     }
    //     i = 0;
    //     x1 = 0;
    //     y1++;
    // }
    // for (int i = 0; i < BoardSize; i++)//wype?nianie tablicy: znaki pól
    // {
    //     cel[i].z = 'I';
    // }
    // for (int i = 0; i < BoardSize; i++)//wype?nianie tablicy: znaki pól
    // {
    //     cel[i].p = i;
    // }
    //
    // //g?ówny kod gry
    // println!("Hello, world!");
}
