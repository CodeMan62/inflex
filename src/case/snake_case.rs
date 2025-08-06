use regex::Regex;
// snake_case
pub fn to_snake_case(s: &str) -> String {
    let re = Regex::new(r"([A-Z]+)([A-Z][a-z])|([a-z\d])([A-Z])").unwrap();
    let s = re.replace_all(s, "${1}${3}_${2}${4}");
    s.replace(['-', ' ', '.'], "_").to_lowercase()
}
// SNAKE_CASE
// incase someone looking for regex version
//pub fn to_screaming_snake_case(s: &str) -> String {
//    let re = Regex::new(r"([A-Z]+)([A-Z][a-z])|([a-z\d])([A-Z])").unwrap();
//    let s = re.replace_all(s, "${1}${3}_${2}${4}");
//    s.replace(['-', ' ', '.'], "_").to_uppercase()
//}
pub fn to_screaming_snake_case(s: &str) -> String {
    let snake_case_statement = to_snake_case(s);
    snake_case_statement.to_uppercase()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("camelCase"), "camel_case");
        assert_eq!(to_snake_case("PascalCase"), "pascal_case");
        assert_eq!(to_snake_case("kebab-case"), "kebab_case");
        assert_eq!(to_snake_case("Title Case"), "title_case");
        assert_eq!(to_snake_case("dot.case"), "dot_case");
        assert_eq!(to_snake_case("UPPERCASE"), "uppercase");
        assert_eq!(to_snake_case("HTTPRequest"), "http_request");
    }
    #[test]
    fn test_to_screaming_snake_case() {
        //only these two are impl. next impl will contain all as upper -> regex it great
        assert_eq!(to_screaming_snake_case("camelCase"), "CAMEL_CASE");
        assert_eq!(to_screaming_snake_case("PascalCase"), "PASCAL_CASE");
        assert_eq!(to_screaming_snake_case("kebab-case"), "KEBAB_CASE");
        assert_eq!(to_screaming_snake_case("Title Case"), "TITLE_CASE");
        assert_eq!(to_screaming_snake_case("dot.case"), "DOT_CASE");
        assert_eq!(to_screaming_snake_case("UPPERCASE"), "UPPERCASE");
        assert_eq!(to_screaming_snake_case("HTTPRequest"), "HTTP_REQUEST");
    }
}
