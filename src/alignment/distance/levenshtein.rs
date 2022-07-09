use std::cmp::min;

pub fn levenshtein(string1: &[u8], string2: &[u8]) -> u32 {
    //create two vectors of integer distance
    let mut prev: Vec<u32> = (0..string1.len() + 1).map(|x| x as u32).collect();
    for row in 0..string2.len() - 1 {
        let mut curr: Vec<u32> = vec![0; string1.len() + 1];
        curr[0] = row as u32 + 1;
        for column in 0..string1.len() {
            let deletion_cost = prev[column + 1] + 1;
            let insertion_cost = curr[column] + 1;
            let substitution_cost = prev[column] + ((string1[column] == string2[row]) as u32);
            curr[column + 1] = min(deletion_cost, min(insertion_cost, substitution_cost));
        }
        prev = curr;
    }
    return prev[string1.len()];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_tests() {
        assert_eq!(levenshtein("kitten".as_bytes(), "sitting".as_bytes()), 3);
        assert_eq!(levenshtein("saturday".as_bytes(), "sunday".as_bytes()), 3);
    }
}
