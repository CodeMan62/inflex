extern crate inflex;
use inflex::Inflex;

fn main() {
    let statement: &str = "fooBar";
    let snake_case = statement.to_snake_case();
    println!("{}", snake_case);
    // foo_bar
}