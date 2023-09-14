// Help elf fix rucksacks. Rucksacks have two compartments of equal size. All items of a given type are meant to go into exactly one of the two compartments. 
// To help prioritize item rearrangement, every item type can be converted to a priority:
// Lowercase item types a through z have priorities 1 through 26. Uppercase item types A through Z have priorities 27 through 52.
// Find the item type that appears in both compartments of each rucksack. Sum this item for each rucksack.

use std::fs;

pub fn get_prio(char: &str) -> usize {
    let is_uppercase = char == char.to_uppercase();
    let char = char.to_lowercase();
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    if is_uppercase {
        return alphabet.find(&char).unwrap() + 27
    } else {
        return alphabet.find(&char).unwrap()+1
    }
}

pub fn rucksacks(input: &str) -> String {
    let mut res = 0;
    
    for sack in input.split("\n") {
        let v: Vec<&str> = sack.split("").collect();
        'inner: for i in 1..v.len()/2 {
            if v[v.len()/2..v.len()].contains(&v[i]) {
                res += get_prio(v[i]);
                break 'inner;
            }
        }
    };
    return res.to_string()
}

pub fn ruck2(input: &str) -> String {
    let mut res: usize = 0;
    let v: Vec<&str> = input.split("\n").collect();

    let mut i = 0;
    
    while i < v.len() {
        let ruck: Vec<&str> = v[i].split("").collect();
        'inner: for j in 1..ruck.len() {
            println!("{} - {}", j, ruck[j]);
            if v[i+1].contains(ruck[j]) && v[i+2].contains(ruck[j]) {
                res += get_prio(ruck[j]);
                break 'inner;
            }
        }
        i += 3;
    }
    
    return res.to_string()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    //println!("{}", rucksacks(&file));
    println!("{}", ruck2(&file));
}
