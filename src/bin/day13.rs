use aoc::intcode::{Program, Int, VM, State};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter;

fn display(vm: &mut VM, board: &mut HashMap<(Int, Int), Int>) {
    for (x, y, id) in iter::from_fn(|| {
        match vm.next_state().unwrap() {
            State::AwaitingInput => None,
            State::Outputting(i) => Some(i),
            State::Finished => None,
        }
    })
        .tuples::<(Int, Int, Int)>()
    {
        board.insert((x, y), id);
    }
}

fn print_board(board: &HashMap<(Int, Int), Int>, w: Int, h: Int) {
    for y in 0..h {
        for x in 0..w {
            print!("{}", match board.get(&(x, y)) {
                Some(0) => ' ',
                Some(1) => '#',
                Some(2) => 'X',
                Some(3) => '-',
                Some(4) => 'o',
                _ => ' ',
            })
        }
        println!();
    }
    println!("Score: {}", board.get(&(-1, 0)).unwrap_or(&0));
    println!();
}

const PRINT_BOARD: bool = false;

const WIDTH: Int = 64;
const HEIGHT: Int = 24;

fn main() {
    let program = Program::from_stdin().unwrap();
    let mut board;
    board = HashMap::new();
    display(&mut VM::of(&program), &mut board);
    let block_count = board.iter().filter(|(_, &id)| id == 2).count();
    println!("Blocks: {}", block_count);

    let mut game = VM::of(&program);
    game.mem[0] = 2;
    board = HashMap::new();

    display(&mut game, &mut board);
    if PRINT_BOARD {
        print_board(&board, WIDTH, HEIGHT);
    }
    while board.iter().filter(|(_, &id)| id == 2).count() > 0 {
        let paddle_x = board.iter().filter(|(_, &id)| id == 3).next().unwrap().0.0;
        let ball_x = board.iter().filter(|(_, &id)| id == 4).next().unwrap().0.0;
        game.input(ball_x.cmp(&paddle_x) as Int);
        display(&mut game, &mut board);
        if PRINT_BOARD {
            print_board(&board, WIDTH, HEIGHT);
        }
    }
    println!("Score: {}", board.get(&(-1, 0)).unwrap_or(&0));
}
