extern crate rayon;

use std::io::{stdin, stdout, BufRead, Write};
use std::convert::TryFrom;
use rayon::prelude::*;

mod sudoku;

use sudoku::Sudoku;

struct SudokuStdinIterator {
    line: String,
}

impl SudokuStdinIterator {
    fn new() -> Self {
        Self {
            line: String::with_capacity(82),
        }
    }
}

impl Iterator for SudokuStdinIterator {
    type Item = Result<Sudoku, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let stdin = stdin();
        let mut lock = stdin.lock();
        
        self.line.clear();
        if let Err(_) = lock.read_line(&mut self.line) {
            return None;
        }

        if self.line.len() != 82 {
            return None;
        }

        Some(Sudoku::try_from(&self.line))
    }
}

fn main() {
    let sudoku_reader = SudokuStdinIterator::new();
    let mut sudoku_list = sudoku_reader.collect::<Vec<_>>();

    sudoku_list.par_iter_mut()
    .map(|parse_result| {
        let solved = match parse_result {
            Ok(ref mut sudoku) => sudoku.solve(),
            _ => false,
        };

        (parse_result, solved)
    })
    .for_each(|(parse_result, solved)| {
        match parse_result {
            Err(_) => {
                println!("Instance unsolveable");
            },
            Ok(sudoku) => {
                let stdout = stdout();
                let mut out_lock = stdout.lock();
            
                let mut print_buffer = [0u8; 82];
                print_buffer[81] = '\n' as u8;
    

                if solved {
                    let field = sudoku.field();

                    for i in 0..81 {
                        let n = field[i];
                        if n == 0 {
                            print_buffer[i] = '0' as u8;
                        } else {
                            print_buffer[i] = n.trailing_zeros() as u8 + '0' as u8 + 1;
                        }
                    }

                    out_lock.write_all(&print_buffer).unwrap();
                } else {
                    println!("No solution found");
                }
            }
        }
    });
}

