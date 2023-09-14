use std::{fs};

fn process_input(input: &str) -> String {
    let res = &input.split("\n\n").map(|elf| {
        elf.lines().map(|food| food.parse::<u32>().expect("parse wrong")).sum::<u32>()
    }).max().expect("max wrong");

    res.to_string()
}

fn p2_process_input(input: &str) -> String {
    let mut res: Vec<u32> = input.split("\n\n").map(|elf| {
        elf.lines().map(|food| food.parse::<u32>().expect("parse wrong")).sum::<u32>()
    }).collect::<Vec<u32>>();
    res.sort_by(|a, b| b.cmp(a));
    
    let elf_0 = res.get(0).unwrap();
    let elf_1 = res.get(1).unwrap();
    let elf_2 = res.get(2).unwrap();

    (elf_0+elf_1+elf_2).to_string()

}

fn main() {
    let file = fs::read_to_string("input.txt").expect("file wrong");
    println!("{}", process_input(&file));
    println!("{}", p2_process_input(&file));
}
