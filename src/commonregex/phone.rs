use super::matcher;

const FR_REGEX: &'static str = r#"(?:(?:\+|00)33|0)\s*[1-9](?:[\s.-]*\d{2}){4}"#;

/// Returns matched French phone numbers as a vector of strings
///
/// # Arguments
///
/// * `text` - A String representing the text in which to search.
///
/// # Remarks
///
/// The digits in the phones can have 3 formats :
///
/// * Not separated : 0606060606
/// * Separated by dots : 06.06.06.06.06
/// * Separated by spaces : 06 06 06 06 06
///
/// # Examples
///
/// ```
/// extern crate commonregex_rs;
///
/// use commonregex_rs::commonregex;
///
/// let text = String::from("On se rejoint au café ? Voici mon num 0606060606");
///
/// assert_eq!(vec!["0606060606"], commonregex::phone::fr(&text));
/// ```
pub fn fr(text: &String) -> Vec<&str> {
    matcher::match_results(text, FR_REGEX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_fr() {
        let text = String::from("On se rejoint au café ? Voici mon num 0606060606");
        assert_eq!(vec!["0606060606"], fr(&text));
    }

    #[test]
    fn phone_fr_spaces() {
        let text = String::from("On se rejoint au café ? Voici mon num 06 06 06 06 06");
        assert_eq!(vec!["06 06 06 06 06"], fr(&text));
    }

    #[test]
    fn phone_fr_dots() {
        let text = String::from("On se rejoint au café ? Voici mon num 06.06.06.06.06");
        assert_eq!(vec!["06.06.06.06.06"], fr(&text));
    }
}
