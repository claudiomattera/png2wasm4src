// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

pub fn sanitize_variable_name(name: &str) -> String {
    name.to_ascii_uppercase()
        .replace("-", "_")
        .chars()
        .skip_while(|c| !c.is_ascii_alphabetic() && *c != '_')
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_name() {
        let sanitized_name = sanitize_variable_name("some_variable");
        let expected = "SOME_VARIABLE";
        assert_eq!(sanitized_name, expected);
    }

    #[test]
    fn sane_name_leading_with_underscore() {
        let sanitized_name = sanitize_variable_name("__some_variable");
        let expected = "__SOME_VARIABLE";
        assert_eq!(sanitized_name, expected);
    }

    #[test]
    fn name_with_space() {
        let sanitized_name = sanitize_variable_name("some variable");
        let expected = "SOMEVARIABLE";
        assert_eq!(sanitized_name, expected);
    }

    #[test]
    fn name_leading_with_digit() {
        let sanitized_name = sanitize_variable_name("123some_variable");
        let expected = "SOME_VARIABLE";
        assert_eq!(sanitized_name, expected);
    }

    #[test]
    fn name_with_invalid_characters() {
        let sanitized_name = sanitize_variable_name("some*variable^");
        let expected = "SOMEVARIABLE";
        assert_eq!(sanitized_name, expected);
    }

    #[test]
    fn name_with_non_ascii_characters() {
        let sanitized_name = sanitize_variable_name("sømæ_våriablæ");
        let expected = "SM_VRIABL";
        assert_eq!(sanitized_name, expected);
    }
}
