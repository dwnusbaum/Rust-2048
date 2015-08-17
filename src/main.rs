extern crate rand;

use rand::Rng;
use std::io;

use Direction::{North, East, South, West};

enum Direction {
    North,
    East,
    South,
    West
}

fn char_to_direction(c : char) -> Option<Direction> {
    match c {
        'w' => Some(North),
        'd' => Some(East),
        's' => Some(South),
        'a' => Some(West),
        _   => None
    }
}

type Row   = Vec<i32>;
type Board = Vec<Row>;

fn slide_left(board : &Board) -> Board {
    return board.iter().map(|r| slide_row_left(r)).collect();
}

fn slide_row_left (row : &Row) -> Row {
    let mut out  = vec!();
    let mut last = row[0];

    for &elem in &row[1..] {
        let pair = (last, elem);
        match pair {
            (0, x) => {
                last = x;
            },
            (_, 0) => {
                continue;
            }
            (x, y) if x == y => {
                out.push(x + y);
                last = 0;
            },
            (x, y) => {
                out.push(x);
                last = y;
            }
        }
    }

    out.push(last);

    while out.len() < 4 {
        out.push(0);
    }

    return out;
}

fn transpose(board : &Board) -> Board {
    let mut out = vec!();
    for column in (0..4) {
        let mut new_row = vec!();
        for row in (0..4) {
            new_row.push(board[row][column]);
        }
        out.push(new_row);
    }
    return out;
}

fn flip_board(board : &Board) -> Board {
    return board.iter().map(|r| r.iter().rev().map(|&x| x).collect()).collect();
}

fn slide(dir : Direction, board : &Board) -> Board {
    match dir {
        North => return transpose(&slide_left(&transpose(board))),
        East  => return flip_board(&slide_left(&flip_board(board))),
        South => return transpose(&flip_board(&slide_left(&flip_board(&transpose(board))))),
        West  => return slide_left(board)
    }
}

fn stalled(board : &Board) -> bool {
    for row in board {
        if !stalled_row(row) {
            return false;
        }
    }

    for row in &transpose(board) {
        if !stalled_row(row) {
            return false;
        }
    }

    return true;
}

fn stalled_row(row : &Row) -> bool {
    if row.iter().map(|&x| x).filter(|&x| x != 0).count() < 4 {
        return false;
    } else {
        let mut last = row[0];
        for &elem in &row[1..] {
            let pair = (last, elem);
            match pair {
                (x, y) if x == y => {
                    return false;
                },
                (_, y) => {
                    last = y;
                }
            }
        }
    }

    return true;
}

fn empty_tiles(board : &Board) -> Vec<(usize,usize)> {
    let mut out = vec!();
    for row in 0..4 {
        for column in 0..4 {
            if board[row][column] == 0 {
                out.push((row, column));
            }
        }
    }
    return out;
}

fn add_tile(board : Board) -> Board {
    let mut rng = rand::thread_rng();
    let mut board = board;
    let tiles = empty_tiles(&board);
    let (row, column) = tiles[rng.gen_range(0, tiles.len())];
    let value = match rng.gen_range(1, 11) {
        1 ... 9 => 2,
        _       => 4
    };
    board[row][column] = value;
    return board;
}

fn print_board(board : &Board) -> () {
    println!("---------------------");
    for row in board {
        for &elem in row {
            match elem {
                0 => print!("|    "),
                _ => print!("|{:^4.3?}", elem)
            }
        }
        println!("|");
    }
    println!("---------------------");
}

fn read_char() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    return input.chars().next().unwrap();
}

fn gameloop(board : Board) -> () {
    print_board(&board);
    if stalled(&board) {
        println!("Game over!");
    } else {
        if let Some(dir) = char_to_direction(read_char()) {
            let board1 = slide(dir, &board);
            if board1 == board {
                gameloop(board);
            } else {
                let board2 = add_tile(board1);
                gameloop(board2);
            }
        } else {
            gameloop(board);
        }
    }
}

fn main() {
    let row = vec!(0,0,0,0);
    let board = vec!(row.clone(), row.clone(), row.clone(), row.clone());
    let board1 = add_tile(board);
    let board2 = add_tile(board1);
    gameloop(board2);
}
