use std::{process::Command, io::{self, Write}};


fn main() {
    let mut board: Vec<Vec<i32>> = vec![vec![0,0,0], vec![0,0,0], vec![0,0,0]];
    let mut turn: i32 = 0;
    clear();
    loop {
        print_board(board.clone());
        let win = check_for_win(board.clone());
        if win != 0 {
            println!("Player: {}, has won the game", player_char(win));
            return;
        }

        let mut action = String::new();
        print!("Please enter the number for the slot you want: ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut action);
        if !check_slot(board.clone(), action.trim()) {
            clear();
            println!("That slot is already taken, try again");
            continue;
        }
        if !is_string_numeric(action.trim().to_string()) {
            clear();
            println!("That was not a valid slot number, try again");
            continue;
        }
        board = update_board(board, action.trim(), turn);
        turn += 1;
        clear();

    }
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn update_board(mut board: Vec<Vec<i32>>, action: &str, turn: i32) -> Vec<Vec<i32>> {
    let whos: i32 = turn % 2;
    let args: Vec<char> = action.chars().collect();
    let row_num = args[0].to_string().parse::<i32>().unwrap() as usize;
    let slot_num = args[1].to_string().parse::<i32>().unwrap() as usize;
    
    board[row_num][slot_num] = whos + 1;
    return board;
}

fn print_board(board: Vec<Vec<i32>>) {
    let mut board_format = "\n      0 1 2\n 0 -> ".to_owned();
    for row in 0..3 {
        for slot in 0..3 {
            let position = board[row][slot];
            let character = player_char(position);
            board_format = board_format + character + " ";
        }
        if row <= 1 {
            board_format = board_format + format!("\n {} -> ", row + 1).as_str();
        }
    }

    println!("{board_format}");
}

fn check_slot(board: Vec<Vec<i32>>, action: &str) -> bool {

    let args: Vec<char> = action.chars().collect();
    let row_num = args[0].to_string().parse::<i32>().unwrap() as usize;
    let slot_num = args[1].to_string().parse::<i32>().unwrap() as usize;
    return &board[row_num][slot_num] == &0;
}

fn clear() {
    let _ = Command::new("cmd.exe").arg("/c").arg("cls").status();
}

fn check_for_win(board: Vec<Vec<i32>>) -> i32 {

    let l_t_r = scan_left_to_right(board.clone());
    let l_t_r_d = scan_left_to_right_diagional(board.clone());
    let r_t_l_d = scan_right_to_left_diagional(board.clone());
    let t_t_b = scan_top_to_bottom(board.clone());


    if l_t_r != 0 {
        return l_t_r;
    }

    if l_t_r_d != 0 {
        return l_t_r_d;
    }
    
    if r_t_l_d != 0 {
        return r_t_l_d;
    }

    if t_t_b != 0 {
        return t_t_b;
    }

    return 0;
}

fn scan_left_to_right(board: Vec<Vec<i32>>) -> i32 {
    for i in 0..3 {
        let mut row = "".to_owned();
        for j in 0..3 {
            row += board[i][j].to_string().as_str();
        }
        if row == "111" {
            return 1;
        } else if row == "222" {
            return 2;
        }
    }
    return 0;
}

fn scan_left_to_right_diagional(board: Vec<Vec<i32>>) -> i32 {
    let mut row = "".to_owned();
    for i in 0..3 {
        row += board[i][i].to_string().as_str();
        
    }
    if row == "111" {
        return 1;
    } else if row == "222" {
        return 2;
    }
    return 0;
}

fn scan_right_to_left_diagional(board: Vec<Vec<i32>>) -> i32 {
    let mut row = "".to_owned();
    for i in 0..3 {
        row += board[2-i][i].to_string().as_str();
    }
    if row == "111" {
        return 1;
    } else if row == "222" {
        return 2;
    }
    return 0;
}

fn scan_top_to_bottom(board: Vec<Vec<i32>>) -> i32 {
    for i in 0..3 {
        let mut row = "".to_owned();
        for j in 0..3 {
            row += board[j][i].to_string().as_str();
        }
        if row == "111" {
            return 1;
        } else if row == "222" {
            return 2;
        }
    }
    return 0;
}

fn player_char(number: i32) -> &'static str {
    match number {
        1=> return "x",
        2=> return "o",
        _=> return "-",
    }

    
}