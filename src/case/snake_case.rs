use regex::Regex;

pub fn to_snake_case(s: &str) -> String {
    let re = Regex::new(r"([A-Z]+)([A-Z][a-z])|([a-z\d])([A-Z])").unwrap();
    let s = re.replace_all(s, "${1}${3}_${2}${4}");
    s.replace(['-', ' ', '.'], "_").to_lowercase()
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
}
