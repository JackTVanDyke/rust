use std::fs;

pub fn score(p1: char, p2: char) -> u32 {
    let mut res: u32 = 0;
    
    if p1 == 'A' && p2 == 'X' {
        res = 4;
    } else if p1 == 'A' && p2 == 'Y' {
        res = 8;
    } else if p1 == 'A' && p2 == 'Z' {
        res = 3;
    } else if p1 == 'B' && p2 == 'X' {
        res = 1;
    } else if p1 == 'B' && p2 == 'Y' {
        res = 5;       
    } else if p1 == 'B' && p2 == 'Z' {
        res = 9;
    } else if p1 == 'C' && p2 == 'X' {
        res = 7;
    } else if p1 == 'C' && p2 == 'Y' {
        res = 2;
    } else if p1 == 'C' && p2 == 'Z' {
        res = 6;
    }
    res
}

// a, x -> lose, rock
// b, y -> draw, paper
// c, z -> win, scissors
pub fn cheat(p1: char, p2: char) -> char {
    let mut res = 'f';
        if p1 == 'A' && p2 == 'X' {
        res = 'Z';
    } else if p1 == 'A' && p2 == 'Y' {
        res = 'X';
    } else if p1 == 'A' && p2 == 'Z' {
        res = 'Y';
    } else if p1 == 'B' && p2 == 'X' {
        res = 'X';
    } else if p1 == 'B' && p2 == 'Y' {
        res = 'Y';       
    } else if p1 == 'B' && p2 == 'Z' {
        res = 'Z';
    } else if p1 == 'C' && p2 == 'X' {
        res = 'Y';
    } else if p1 == 'C' && p2 == 'Y' {
        res = 'Z';
    } else if p1 == 'C' && p2 == 'Z' {
        res = 'X';
    }
    

    res
}

pub fn rps(input: &str) -> String {
    let mut res = 0;
    input.split("\n").for_each(|game| {
        let char1 = game.chars().nth(0).expect("char1 wrong");
        let char2 = game.chars().nth(2).expect("char2 wrong");
        res += score(char1, char2);
    });
    
    res.to_string()
}

pub fn rps2(input: &str) -> String {
    let mut res = 0;
    input.split("\n").for_each(|game| {
        let char1 = game.chars().nth(0).expect("char1 wrong");
        let char2 = game.chars().nth(2).expect("char2 wrong");
        res += score(char1, cheat(char1, char2));
    });
    
    res.to_string()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("file wrong");
    println!("{}", rps2(&file));
}

