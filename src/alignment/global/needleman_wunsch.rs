//! Space-optimized implementation of the Needleman-Wunsch global alignment algorithm.
use core::panic;
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
    value: u32,
    source: Source,
}

impl Entry {
    fn new(value: u32, source: Source) -> Self {
        Self { value, source }
    }
    fn default() -> Self {
        Entry::new(0_u32, Source::Default)
    }
}

struct Aligner {
    alignment1: String,
    alignment2: String,
    row: usize,
    col: usize,
}

impl Aligner {
    fn new(length1: usize, length2: usize) -> Self {
        Self {
            alignment1: String::with_capacity(max(length1, length2)),
            alignment2: String::with_capacity(max(length1, length2)),
            row: length1,
            col: length2,
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

/// The algorithm has both a time and space complexity of O(mn), where m and n are input sequence lengths.
///
/// Takes str slices of two sequences as inputs, and returns a tuple of two strings representing the sequences with optimal alignment.
///
/// Note on space-efficiency: This implementation of the algorithm is designed for use with larger nucleotide and amino acid sequences
/// as it dynamically compresses past rows of the matrix such that each cell of the compressed matrix only takes up 1 byte in memory.
///
/// Note on gap scoring: This algorithm seeks to model biological systems, so it is implemented such to identify the number of individual
/// mutation events that would have to occur to turn seq1 into seq2. Notably, the algorithm treats consecutive insertions/deletions as single mutation events.
///
/// # Examples
/// ```
/// use bioinformatics::alignment::global::needleman_wunsch::needleman_wunsch;
///
/// let (string1, string2) = needleman_wunsch("similarity", "molarity");
/// assert_eq!(&string1,"similarity");
/// assert_eq!(&string2,"--molarity");
///
///
/// let (string3, string4) = needleman_wunsch("GAAAATAAAT", "GATAAT");
/// assert_eq!(&string3,"GAAAATAAAT");
/// assert_eq!(&string4,"G---AT-AAT");
/// ```
pub fn needleman_wunsch(seq1: &str, seq2: &str) -> (String, String) {
    //seq1 will always be larger
    if seq1.len() < seq2.len() {
        return needleman_wunsch(seq2, seq1);
    }
    let seq1 = seq1.as_bytes();
    let seq2 = seq2.as_bytes();
    //establish matrix dimensions from input
    let rows = seq1.len() + 1;
    let cols = seq2.len() + 1;
    //allocate contiguous block of memory for mutable matrix
    let mut matrix_linear = vec![Source::Default; rows * cols];
    let mut matrix_grid: Vec<_> = matrix_linear.as_mut_slice().chunks_mut(cols).collect();
    let matrix = matrix_grid.as_mut_slice();
    //generate temp vectors for calculations
    let mut prev = vec![Entry::default(); cols];
    let mut curr = vec![Entry::default(); cols];
    //initialize gap penalties from empty strings
    for col in 0..cols {
        prev[col] = Entry::new(col as u32, Source::Insertion);
    }
    //begin filling table
    for row in 1..rows {
        curr = vec![Entry::default(); cols];
        curr[0] = Entry::new(row as u32, Source::Deletion);
        for col in 1..cols {
            let matchmismatch = Entry::new(
                prev[col - 1].value + ((seq1[row - 1] != seq2[col - 1]) as u32),
                Source::MatchMismatch,
            );
            let insertion = Entry::new(
                curr[col - 1].value + ((curr[col - 1].source != Source::Insertion) as u32),
                Source::Insertion,
            );
            let deletion = Entry::new(
                prev[col].value + ((prev[col].source != Source::Deletion) as u32),
                Source::Deletion,
            );
            if matchmismatch.value <= insertion.value && matchmismatch.value <= deletion.value {
                curr[col] = matchmismatch;
            } else if deletion.value <= insertion.value {
                curr[col] = deletion;
            } else {
                curr[col] = insertion;
            }
        }
        for col in 0..cols {
            matrix[row - 1][col] = prev[col].source;
        }
        if row == rows - 1 {
            for col in 0..cols {
                matrix[rows - 1][col] = curr[col].source;
            }
        }
        prev = curr;
    }
    //compute alignment
    let mut aligner = Aligner::new(seq1.len(), seq2.len());
    while aligner.row > 0 || aligner.col > 0 {
        match matrix[aligner.row][aligner.col] {
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
    }
    return (
        aligner.alignment1.chars().rev().collect::<String>(),
        aligner.alignment2.chars().rev().collect::<String>(),
    );
}
