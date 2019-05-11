use super::regex_drawer;
use super::matcher;

/// Returns matched IPv4 addresses as a vector of strings
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
/// let log = String::from("
/// Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35
/// Started GET '/' for 10.10.0.1 at 2019-05-11 00:52:05");
/// 
/// assert_eq!(vec!["127.0.0.1", "10.10.0.1"], commonregex::ip::v4(&log));
/// ```
pub fn v4(text: &String) -> Vec<&str> {
    matcher::match_results(text, regex_drawer::ip::V4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4() {
        let text = String::from("Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35");
        assert_eq!(vec!["127.0.0.1"], v4(&text));
    }
}