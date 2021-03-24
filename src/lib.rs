use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output = HashSet::new();
    let letters = word.to_lowercase().chars().collect::<Vec<char>>();
    'outer: for possible in possible_anagrams {
        if possible.to_lowercase() == word.to_lowercase() {
            continue
        }
        let mut test_letters = letters.clone();
        let mut test_possible = possible.to_lowercase().chars().collect::<Vec<char>>();
        loop {
            if let Some(index) = test_letters.iter().position(|x| *x == test_possible[0]) {
                test_letters.remove(index);
                test_possible.remove(0);
                if test_letters == test_possible {
                    output.insert(*possible);
                } else if test_possible == Vec::new(){
                    break;
                }
           } else {
             break 
           }
        }
    }
    output
}
