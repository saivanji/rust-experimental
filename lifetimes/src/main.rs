fn main() {
    println!("Hello, world!");
}

pub fn concat<'a>(a: &'a str, b: &'a str) -> &'a str {
    a
}
