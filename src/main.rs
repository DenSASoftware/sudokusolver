use std::io::*;
use std::convert::TryFrom;

mod sudoku;

use sudoku::Sudoku;

fn main() {
    let stdin = stdin();
    let mut lock = stdin.lock();

    let stdout = stdout();
    let mut out_lock = stdout.lock();

    let mut line = String::with_capacity(82);
    let mut print_buffer = [0u8; 82];
    print_buffer[81] = '\n' as u8;
    
    'mainloop: loop {
        line.clear();

        if let Err(_) = lock.read_line(&mut line) {
            break;
        }

        // This will always fail under windows, just as it deserves for using \r\n as newline
        if line.len() != 82 {
            if line.len() == 0 { // EOF
                return;
            }
            eprintln!("Line doesn't contain 81 characters plus newline, found {:?}", line);

            continue;
        }

        match Sudoku::try_from(&line) {
            Err(_) => {
                println!("Instance unsolveable");
            },
            Ok(mut sudoku) => {
                if sudoku.solve() {
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
    }
}

