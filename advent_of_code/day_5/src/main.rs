use std::fs;

pub fn parse_file(input: &str) {

}

// pub fn organize(input: &str) -> String {
//     let mut res = String::new();

//     return res
// }

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", file);
}
