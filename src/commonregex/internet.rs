use super::regex_drawer;
use super::matcher;

/// Returns matched email addresses as a vector of strings
///
/// # Arguments
///
/// * `text` - A String representing the text in which to search.
///
/// # Examples
///
/// ```
/// extern crate commonregex_rs;
///
/// use commonregex_rs::commonregex;
///
/// let text = String::from("I'm the Doctor. Contact me at hello@tardis.com or hello@gallifrey.com.");
///
/// assert_eq!(vec!["hello@tardis.com", "hello@gallifrey.com"], commonregex::internet::email(&text));
/// ```
pub fn email(text: &String) -> Vec<&str> {
    matcher::match_results(text, regex_drawer::internet::EMAIL)
}

/// Returns matched URLs as a vector of strings
///
/// # Arguments
///
/// * `text` - A String representing the text in which to search.
///
/// # Examples
///
/// ```
/// extern crate commonregex_rs;
///
/// use commonregex_rs::commonregex;
///
/// let text = String::from("Contribute at https://github.com/fakenine/commonregex-rs !");
///
/// assert_eq!(vec!["https://github.com/fakenine/commonregex-rs"], commonregex::internet::url(&text));
/// ```
pub fn url(text: &String) -> Vec<&str> {
    matcher::match_results(text, regex_drawer::internet::URL)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_emails() {
        let text = String::from("I'm the Doctor. Contact me at hello@tardis.com or hello@gallifrey.com.");
        assert_eq!(vec!["hello@tardis.com", "hello@gallifrey.com"], email(&text));
    }
}