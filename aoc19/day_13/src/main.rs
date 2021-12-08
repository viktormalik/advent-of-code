extern crate termion;

use intcode::*;
use itertools::Itertools;
use std::env;
use std::fs;
use std::io::{stdout, Write};
use std::thread;
use std::time;
use termion::raw::IntoRawMode;

struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<char>>,
    score: i64,
    ball: usize,
    paddle: usize,
}

impl Game {
    fn get_tiles(output: &Vec<i64>) -> Vec<Vec<i64>> {
        output
            .iter()
            .chunks(3)
            .into_iter()
            .map(|c| c.map(|x| *x).collect())
            .collect()
    }

    fn initialize(output: &Vec<i64>) -> Game {
        let tiles = Game::get_tiles(&output);

        let width = tiles.iter().map(|t| t[0]).max().unwrap() as usize + 1;
        let height = tiles.iter().map(|t| t[1]).max().unwrap() as usize + 1;

        Game {
            width: width,
            height: height,
            board: vec![vec![' '; width]; height],
            score: 0,
            paddle: 0,
            ball: 0,
        }
    }

    fn update(&mut self, output: &Vec<i64>) {
        let tiles = Game::get_tiles(&output);
        for t in &tiles {
            if t[0] == -1 {
                self.score = t[2];
            } else {
                let x = t[0] as usize;
                let y = t[1] as usize;
                self.board[y][x] = match t[2] {
                    1 => '#',
                    2 => '@',
                    3 => '_',
                    4 => 'o',
                    _ => ' ',
                };
                if t[2] == 3 {
                    self.paddle = x;
                } else if t[2] == 4 {
                    self.ball = x;
                }
            }
        }
    }

    fn print_board(&self) {
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        let mut row = 1;
        for y in 0..self.height {
            write!(stdout, "{}", termion::cursor::Goto(1, row)).unwrap();
            for x in 0..self.width {
                print!("{}", self.board[y][x]);
            }
            row += 1;
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut program = IntcodeProgram::load(&input);
    program.run(&vec![]);

    let blocks = program
        .stdout
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|&x| *x == 2)
        .count();

    let mut game = Game::initialize(&program.stdout);

    let mut program = IntcodeProgram::load(&input);
    let mut joystick = 0;
    program.set_mem(0, 2);

    while program.state != ProgramState::Term {
        program.run(&vec![joystick]);
        game.update(&program.stdout);

        if env::args().any(|a| a == "print") {
            game.print_board();
        }

        if game.ball != game.paddle {
            joystick = game.ball as i64 - game.paddle as i64;
            joystick /= joystick.abs();
        }
    }

    println!();
    println!("First: {}", blocks);
    println!("Second: {}", game.score);
}
