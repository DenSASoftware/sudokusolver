use std::convert::TryFrom;

// This function should be a method on `Sudoku`, but the functionality is needed elsewhere too
fn remove_adjacent(notes: &mut [i32; 81], index: usize, candidate: i32) {
    let row = (index / 9) * 9;
    let col = index % 9;
    let square = (index / 27) * 27 + (col / 3) * 3;

    let mask = !candidate;
    for i in 0..9 {
        notes[row + i] &= mask;
        notes[col + i * 9] &= mask;
    }

    for offset in [0, 1, 2, 9, 10, 11, 18, 19, 20].iter() {
        notes[square + offset] &= mask;
    }
}

#[derive(Clone)]
pub struct Sudoku {
    /// The field of a 9x9-sudoku as 1d-array. But instead of storing the number that is (not)
    /// written in this field, a binary number is used instead for easy use with the notes-field.
    /// 0 denotes an empty field
    field: [i32; 81],
    /// The notes for each field. The structure of this array is the same as the `field`-field. The
    /// bits of an element denotes the possible numbers that can be written in this field. The
    /// smallest bit set indicates a 1 might be written in this field, the second smallest bit set
    /// indicates a 2 might be written in this field etc.
    notes: [i32; 81],
}

impl Sudoku {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            field: [0; 81],
            notes: [0x1ff; 81],
        }
    }

    fn from_arrays(field: [i32; 81], notes: [i32; 81]) -> Self {
        Self {
            field,
            notes,
        }
    }

    fn remove_adjacent(&mut self, field: usize, candidate: i32) {
        remove_adjacent(&mut self.notes, field, candidate);
    }

    fn put(&mut self, field: usize, candidate: i32) {
        self.field[field] = candidate;
        self.notes[field] = 0;
        self.remove_adjacent(field, candidate);
    }

    fn singles(&mut self) -> bool {
        let mut found = false;

        for i in 0..81 {
            if self.field[i] != 0 {
                continue;
            }

            let candidates = self.notes[i];
            let count = candidates.count_ones();

            if count == 0 {
                return false;
            }
            if count == 1 {
                self.put(i, candidates);

                found = true;
            }
        }

        found
    }

    #[inline(always)]
    fn hsinglesblock(&mut self, start: usize, inc: usize, skip: usize) -> bool {
        let mut onceormore = 0;
        let mut twiceormore = 0;
        let mut index = start;

        for _i in 0..3 {
            for _j in 0..3 {
                let candidates = self.notes[index];
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
                let intersect = self.notes[index] & once;
                if intersect != 0 {
                    self.put(index, intersect);
                }

                index += inc;
            }

            index += skip;
        }

        true
    }

    fn hsingles(&mut self) -> bool {
        for i in 0..9 {
            let found_something = self.hsinglesblock(i * 9, 1, 0) ||
                self.hsinglesblock(i, 9, 0) ||
                self.hsinglesblock((i / 3) * 27 + (i % 3) * 3, 1, 6);

            if found_something {
                return true;
            }
        }

        false
    }

    pub fn solve(&mut self) -> bool {
        let mut found = true;
        while found {
            found = self.singles() || self.hsingles();
        }

        let mut min = 10;
        let mut mindex: i32 = -1;

        for i in 0..81 {
            let count = self.notes[i].count_ones();
            if self.field[i] == 0 && count < min {
                min = count;
                mindex = i as i32;
            }
        }

        if mindex == -1 {
            return true;
        }

        let clone = self.clone();

        let mut candidates = self.notes[mindex as usize];
        while candidates != 0 {
            let c = candidates & -candidates;
            self.put(mindex as usize, c);

            if self.solve() {
                return true;
            }

            self.field[..].copy_from_slice(&clone.field);
            self.notes[..].copy_from_slice(&clone.notes);

            candidates &= !c;
        }

        false
    }

    pub fn field(&self) -> &[i32; 81] {
        &self.field
    }
}

impl TryFrom<&String> for Sudoku {
    type Error = ();

    fn try_from(line: &String) -> Result<Self, Self::Error> {
        let mut field = [0; 81];
        let mut notes = [0x1ff; 81];

        for (i, ch) in line.chars().enumerate().take(81) {
            if ch > '9' || ch < '0' {
                eprintln!("Illegal digit: {}", ch);

                return Err(());
            }


            let digit = ch as i32 - '0' as i32;
            let bit = if digit > 0 { 1 << (digit - 1) } else { 0 };

            field[i] = bit;
            if digit > 0 {
                if (!notes[i] & bit) != 0 {
                    return Err(());
                }

                notes[i] = 0;
                remove_adjacent(&mut notes, i, bit);
            }
        }

        Ok(Sudoku::from_arrays(field, notes))
    }
}

