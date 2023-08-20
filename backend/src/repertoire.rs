use std::{
    fs,
    ops,
    collections::hash_set::HashSet,
};

pub struct Repertoire {
    words: Vec<String>,
    irs: Vec<u32>,
    letters_bags: Vec<u8>,
    words_map: HashSet<String>,
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).unwrap();

    content.lines().map(|l| String::from(l)).collect()
}

fn get_ir(word: &str) -> u32 {
    let mut ir : u32 = 0;
    for i in word.chars() {
        ir |= 0x80000000 >> (i as u8 - 'A' as u8);
    }
    return ir;
}

fn get_letters(letters :&str, letters_bag: &mut [u8]) {
    for i in letters.chars() {
        letters_bag[i as usize - 'A' as usize] += 1;
    }
}

fn is_feasible(test :&[u8], against: &[u8]) -> bool {
    for i in 0..26 {
        if test[i] < against[i] {
            return false;
        }
    }

    return true;
}


impl ops::Index<usize> for Repertoire {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.words[index];
    }
}


impl Repertoire {
    pub fn new(file: &str) -> Repertoire {
        let words = lines_from_file(file);
        let irs : Vec<_> = words.iter().map(|word| get_ir(word)).collect();
        let mut letters_bags : Vec<u8> = vec![0; words.len() * 26];

        for i in 0..words.len() {
            get_letters(&words[i], &mut letters_bags[26 * i..26 * (i + 1)])
        }

        let mut words_map = HashSet::with_capacity(words.len());

        for i in &words {
            words_map.insert(i.clone());
        }

        return Repertoire {
            words,
            irs,
            letters_bags,
            words_map
        }
    }

    pub fn filter(&self, letters : &str) -> Vec<usize> {
        let mut ret : Vec<usize> = Vec::with_capacity(self.words.len());

        let letter_ir = get_ir(letters);
        let letter_count = letters.len();

        for i in 0..self.words.len() {
            if !(!self.irs[i] | letter_ir) == 0 && letter_count >= self.words[i].len() {
                ret.push(i);
            }
        }

        let mut letters_bag : [u8; 26] = [0; 26];
        get_letters(letters, &mut letters_bag);
        let mut valide = 0;
        for i in 0..ret.len() {
            let index = ret[i];
            if is_feasible(&letters_bag, &self.letters_bags[index * 26..(index + 1) * 26]) {
                ret[valide] = index;
                valide += 1;
            }
        }

        ret.resize(valide, 0);
        return ret;
    }

}

