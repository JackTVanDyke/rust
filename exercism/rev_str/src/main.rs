// Reverse a string

fn reverse(input: &str) -> String {
    let result = input.chars().rev().collect::<String>();
    result
}

fn main() {
    let rev_str = reverse("uüu");
    println!("{}", rev_str);

    let rev_str = reverse("fuckthis");
    println!("{}", rev_str);
}
