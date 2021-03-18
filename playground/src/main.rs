fn main() {
    let mut greeting = String::from("Hello");
    let name = String::from("John");

    let text = &mut greeting;

    println!("{}", text);
}
