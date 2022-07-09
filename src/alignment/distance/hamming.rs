pub fn hamming(string1: &[u8], string2: &[u8]) -> u32 {
    if string1.len() == string2.len() {
        string1
            .iter()
            .zip(string2)
            .map(|(a, b)| (a != b) as u32)
            .sum()
    } else {
        panic!(
            "input strings are of unequal length. string1.len = {}\n string2.len = {}",
            string1.len(),
            string2.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_tests() {
        assert_eq!(hamming("karolin".as_bytes(), "kathrin".as_bytes()), 3);
        assert_eq!(hamming("karolin".as_bytes(), "kerstin".as_bytes()), 3);
        assert_eq!(hamming("kathrin".as_bytes(), "kerstin".as_bytes()), 4);
        assert_eq!(hamming("0000".as_bytes(), "1111".as_bytes()), 4);
        assert_eq!(hamming("2173896".as_bytes(), "2233796".as_bytes()), 3);
    }
}
