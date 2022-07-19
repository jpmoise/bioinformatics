// Copyright 2022 John Moise.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed except according to those terms.

use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Source {
    Default,
    MatchMismatch,
    Insertion,
    Deletion,
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    value: i32,
    source: Source,
}

impl Entry {
    fn new(value: i32, source: Source) -> Self {
        Self { value, source }
    }
    fn default() -> Self {
        Entry::new(0_i32, Source::Default)
    }
}

struct Aligner {
    alignment1: String,
    alignment2: String,
    row: usize,
    col: usize,
}

impl Aligner {
    fn new(row: usize, col: usize) -> Self {
        Self {
            alignment1: String::with_capacity(max(row, col)),
            alignment2: String::with_capacity(max(row, col)),
            row,
            col,
        }
    }
    fn shift_matchmismatch(&mut self) {
        self.row -= 1;
        self.col -= 1;
    }
    fn shift_insertion(&mut self) {
        self.col -= 1;
    }
    fn shift_deletion(&mut self) {
        self.row -= 1;
    }
    fn print(&self) {
        println!("{}", self.alignment1.chars().rev().collect::<String>());
        println!("{}", self.alignment2.chars().rev().collect::<String>());
    }
}

pub fn smith_waterman(seq1: &[u8], seq2: &[u8]) {
    //seq1 will always be larger
    if seq1.len() < seq2.len() {
        return smith_waterman(seq2, seq1);
    }
    //establish matrix dimensions from input
    let rows = seq1.len() + 1;
    let cols = seq2.len() + 1;
    //allocate contiguous block of memory for mutable matrix
    let mut matrix_linear = vec![Entry::default(); rows * cols];
    let mut matrix_grid: Vec<_> = matrix_linear.as_mut_slice().chunks_mut(cols).collect();
    let matrix = matrix_grid.as_mut_slice();
    // println!("{:#?}", matrix);
    //initialize gap penalties from empty strings
    for col in 0..cols {
        matrix[0][col] = Entry::new(0_i32, Source::Insertion);
    }
    for row in 0..rows {
        matrix[row][0] = Entry::new(0_i32, Source::Deletion);
    }
    let mut maxvalue = 0;
    let mut maxvalposition = (0, 0);
    for row in 1..rows {
        for col in 1..cols {
            let matchmismatch = Entry::new(
                matrix[row - 1][col - 1].value + 3 * ((seq1[row - 1] == seq2[col - 1]) as i32)
                    - 3 * ((seq1[row - 1] != seq2[col - 1]) as i32),
                Source::MatchMismatch,
            );
            let insertion = Entry::new(
                matrix[row][col - 1].value
                    - 2 * ((matrix[row][col - 1].source != Source::Insertion) as i32),
                Source::Insertion,
            );
            let deletion = Entry::new(
                matrix[row - 1][col].value
                    - 2 * ((matrix[row - 1][col].source != Source::Deletion) as i32),
                Source::Deletion,
            );
            if matchmismatch.value >= insertion.value && matchmismatch.value >= deletion.value {
                if matchmismatch.value >= maxvalue {
                    maxvalue = matchmismatch.value;
                    maxvalposition = (row, col);
                }
                matrix[row][col] = matchmismatch;
            } else if deletion.value >= insertion.value {
                if deletion.value >= maxvalue {
                    maxvalue = deletion.value;
                    maxvalposition = (row, col);
                }
                matrix[row][col] = deletion;
            } else {
                if insertion.value >= maxvalue {
                    maxvalue = insertion.value;
                    maxvalposition = (row, col);
                }
                matrix[row][col] = insertion;
            }
        }
    }
    println!("{:#?}", matrix);
    //compute alignment
    let mut aligner = Aligner::new(maxvalposition.0, maxvalposition.1);
    loop {
        match matrix[aligner.row][aligner.col].source {
            Source::MatchMismatch => {
                aligner.alignment1.push(seq1[aligner.row - 1] as char);
                aligner.alignment2.push(seq2[aligner.col - 1] as char);
                aligner.shift_matchmismatch();
            }
            Source::Insertion => {
                aligner.alignment1.push('-');
                aligner.alignment2.push(seq2[aligner.col - 1] as char);
                aligner.shift_insertion();
            }
            Source::Deletion => {
                aligner.alignment1.push(seq1[aligner.row - 1] as char);
                aligner.alignment2.push('-');
                aligner.shift_deletion();
            }
            Source::Default => {
                panic!("Error in computing matrix.");
            }
        }
        if matrix[aligner.row][aligner.col].value == 0 {
            break;
        };
    }
    aligner.print();
}
