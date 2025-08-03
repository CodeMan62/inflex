extern crate inflex;
use inflex::Inflex;

fn main() {
    let statement: &str = "fooBar";
    let snake_case = statement.to_snake_case();
    println!("{}", snake_case);
    // foo_bar
    let statement2: &str = "fooBar";
    let screaming_snake_case = statement2.to_screaming_snake_case();
    println!("{}", screaming_snake_case);
}
