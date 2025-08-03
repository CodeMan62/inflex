pub mod case;
use case::snake_case::to_screaming_snake_case as screaming_snake_case;
use case::snake_case::to_snake_case as snake_case;
pub trait Inflex {
    fn to_snake_case(&self) -> String;
    fn to_screaming_snake_case(&self) -> String;
}

impl Inflex for str {
    fn to_snake_case(&self) -> String {
        snake_case(self)
    }
    fn to_screaming_snake_case(&self) -> String {
        screaming_snake_case(self)
    }
}
