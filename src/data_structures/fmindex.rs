use hashbag::HashBag;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FMIndex {
    pub bwt_l: String,
    char_counts: HashMap<char, Vec<usize>>,
    occ: Vec<u32>,
    pub directory: Vec<usize>,
}

impl FMIndex {
    pub fn new(input: &str) -> Self {
        //calculate the bwt of the given &str
        let (bwt_l, directory) = FMIndex::bwt(input);
        //count the number of each character
        let raw_char_counts: HashBag<char> = bwt_l.chars().fold(HashBag::new(), |mut bag, c| {
            bag.insert(c);
            bag
        });
        //extract keys and initialize char_count hashmap
        let mut keys: Vec<&char> = raw_char_counts.set_iter().map(|(x, _)| x).collect();
        let mut char_counts: HashMap<char, Vec<usize>> = HashMap::new();
        keys.sort();
        //loop through each key and sum the number of lexographically smaller characters, storing in hashmap
        for (index, key) in keys.iter().enumerate().rev() {
            let startindex: usize = keys.iter().take(index).fold(0, |mut acc, k| {
                acc += raw_char_counts.contains(&k);
                acc
            });
            let endindex = startindex + raw_char_counts.contains(&keys[index]);
            char_counts.insert(**key, vec![startindex, endindex]);
        }

        //generate occ(index) by looping through the bwt
        let mut occ: Vec<u32> = Vec::with_capacity(bwt_l.len());
        let mut bag: HashBag<char> = HashBag::new();
        for c in bwt_l.chars() {
            bag.insert(c);
            occ.push(bag.contains(&c) as u32);
        }

        //generate directory

        Self {
            bwt_l,
            char_counts,
            occ,
            directory,
        }
    }
    pub fn count(&self, pattern: &str) -> usize {
        //counts the occurences of pattern in the string
        self.matches(pattern).len()
    }
    pub fn matches(&self, pattern: &str) -> Vec<usize> {
        let firstindex = pattern.chars().nth_back(0).unwrap();
        let indicestosearch = self.char_counts.get(&firstindex).unwrap();
        let mut validindices: Vec<usize> = Vec::new();
        for i in indicestosearch[0]..indicestosearch[1] {
            let mut index = i;
            let mut validity = true;
            for c in pattern.chars().rev().skip(1) {
                if c != self.bwt_l.as_bytes()[index] as char {
                    validity = false;
                    break;
                } else {
                    index = FMIndex::lf_transform(&self, index);
                }
            }
            if validity {
                validindices.push(i);
            }
        }
        // validindices
        validindices.iter().map(|vi| self.directory[*vi]).collect()
    }
    fn lf_transform(&self, index: usize) -> usize {
        let c = self.bwt_l.as_bytes()[index] as char;
        self.char_counts.get(&c).unwrap()[0] + self.occ[index] as usize - 1
    }
    fn bwt(input: &str) -> (String, Vec<usize>) {
        //initialize transform table
        let mut bwt_table: Vec<String> = Vec::new();
        //turn input into string while appending terminal character
        let input_string = format!("{}{}", input, "$");
        //populate table with all possible rotations
        for (i, _) in input_string.char_indices() {
            bwt_table.push(format!(
                "{}{}",
                &input_string[input_string.len() - 1 - i..],
                &input_string[0..input_string.len() - 1 - i]
            ));
        }
        //sort bwt_table alphabetically
        bwt_table.sort();
        //return the last column of the table
        (
            bwt_table
                .iter()
                .map(|s| s.chars().nth_back(0).unwrap())
                .collect::<String>(),
            bwt_table
                .iter()
                .map(|s| (2 * s.len() - s.find("$").unwrap() - 3) % s.len())
                .collect(),
        )
    }
    pub fn ibwt(&self) -> String {
        let input = self.bwt_l.as_str();
        let mut output = String::new();
        let mut index = input.find("$").unwrap();
        for _ in 0..input.len() - 1 {
            index = FMIndex::lf_transform(&self, index);
            let c = input.as_bytes()[index] as char;
            output.push(c);
        }
        output.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let index = FMIndex::new("example");
        // assert_eq!(index.bwt_l, "ard$rcaaaabb".to_owned());
        // assert_eq!(index.lf_transform(8), 5);
        // assert_eq!(inv_bwt(&bwt("example")), "example".to_owned());
        assert_eq!(index.ibwt(), "example".to_owned());
    }
}
