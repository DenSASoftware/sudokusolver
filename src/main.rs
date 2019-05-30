use std::io::*;
use std::mem::uninitialized;

type Sudoku = [i32; 81];
type SudokuData = [i32; 81];

fn remove_adjacent(data: &mut SudokuData, field: usize, candidate: i32) {
    let r = (field / 9) * 9;
    let c = field % 9;
    let sq = (field / 27) * 27 + (c / 3) * 3;

    let mask: i32 = !candidate;
    for i in 0..9 {
        data[r + i] &= mask;
        data[c + i * 9] &= mask;
    }

    for offset in [0, 1, 2, 9, 10, 11, 18, 19, 20].iter() {
        data[sq + offset] &= mask;
    }
}

fn put(sudoku: &mut Sudoku, data: &mut SudokuData, field: usize, candidate: i32) {
    sudoku[field] = candidate;
    data[field] = 0;
    remove_adjacent(data, field, candidate);
}

fn singles(sudoku: &mut Sudoku, data: &mut SudokuData) -> bool {
    let mut found = false;

    for i in 0..81 {
        if sudoku[i] != 0 {
            continue;
        }

        let candidates = data[i];
        let count = candidates.count_ones();

        if count == 0 {
            return false;
        }
        if count == 1 {
            let c = candidates;
            put(sudoku, data, i, c);

            found = true;
        }
    }

    found
}

#[inline(always)]
fn hsinglesblock(sudoku: &mut Sudoku, data: &mut SudokuData, start: usize, inc: usize, skip: usize) -> bool {
    let mut onceormore = 0;
    let mut twiceormore = 0;
    let mut index = start;

    for _i in 0..3 {
        for _j in 0..3 {
            let candidates = data[index];
            twiceormore |= onceormore & candidates;
            onceormore |= candidates;

            index += inc;
        }

        index += skip;
    }

    let once = onceormore & !twiceormore;
    if once == 0 {
        return false;
    }

    index = start;
    for _i in 0..3 {
        for _j in 0..3 {
            let intersect = data[index] & once;
            if intersect != 0 {
                put(sudoku, data, index, intersect);
            }

            index += inc;
        }

        index += skip;
    }

    true
}

fn hsingles(sudoku: &mut Sudoku, data: &mut SudokuData) -> bool {
    for i in 0..9 {
        let found_something = hsinglesblock(sudoku, data, i * 9, 1, 0) ||
                                hsinglesblock(sudoku, data, i, 9, 0) ||
                                hsinglesblock(sudoku, data, (i / 3) * 27 + (i % 3) * 3, 1, 6);

        if found_something {
            return true;
        }
    }

    false
}

fn solve(sudoku: &mut Sudoku, data: &mut SudokuData) -> bool {
    let mut found = true;
    while found {
        found = singles(sudoku, data) || hsingles(sudoku, data);
    }

    let mut min = 10;
    let mut mindex: i32 = -1;

    for i in 0..81 {
        let count = data[i].count_ones();
        if sudoku[i] == 0 && count < min {
            min = count;
            mindex = i as i32;
        }
    }

    if mindex == -1 {
        return true;
    }

    let scopy = sudoku.clone();
    let dcopy = data.clone();

    let mut candidates = data[mindex as usize];
    while candidates != 0 {
        let c = candidates & -candidates;
        put(sudoku, data, mindex as usize, c);

        if solve(sudoku, data) {
            return true;
        }

        sudoku[..].copy_from_slice(&scopy);
        data[..].copy_from_slice(&dcopy);

        candidates &= !c;
    }

    false
}

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

        let mut sudoku = unsafe { uninitialized::<Sudoku>() };
        let mut sudoku_data = [0x1ffi32; 81];

        let mut unsolvable = false;

        for (i, ch) in line.chars().enumerate().take(81) {
            if ch > '9' || ch < '0' {
                eprintln!("Illegal digit: {}", ch);
                
                continue 'mainloop;
            }

            let digit = ch as i32 - '0' as i32;
            let bit = if digit > 0 { 1 << (digit - 1) } else { 0 };

            sudoku[i] = bit;
            if digit > 0 {
                if (!sudoku_data[i] & bit) != 0 {
                    unsolvable = true;
                    break;
                }
                
                sudoku_data[i] = 0;
                remove_adjacent(&mut sudoku_data, i, bit);
            }
        }

        if unsolvable {
            println!("Instance unsolveable");
        } else if solve(&mut sudoku, &mut sudoku_data) {
            for i in 0..81 {
                let n = sudoku[i];
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

