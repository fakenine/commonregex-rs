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
/// let text = String::from("
/// Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35
/// Started GET '/' for 10.10.0.1 at 2019-05-11 00:52:05");
/// 
/// assert_eq!(vec!["127.0.0.1", "10.10.0.1"], commonregex::ip::v4(&text));
/// ```
pub fn v4(text: &String) -> Vec<&str> {
    matcher::match_results(text, regex_drawer::ip::V4)
}

/// Returns matched IPv6 addresses as a vector of strings
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
/// let text = String::from("Started GET '/' for 2001:0db8:0000:85a3:0000:0000:ac1f:8001 at 2019-05-11 00:51:35");
/// 
/// assert_eq!(vec!["2001:0db8:0000:85a3:0000:0000:ac1f:8001"], commonregex::ip::v6(&text));
/// ```
pub fn v6(text: &String) -> Vec<&str> {
    matcher::match_results(text, regex_drawer::ip::V6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4() {
        let text = String::from("Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35");
        assert_eq!(vec!["127.0.0.1"], v4(&text));
    }

    #[test]
    fn ipv6() {
        let text = String::from("Started GET '/' 2001:0db8:0000:85a3:0000:0000:ac1f:8001 at 2019-05-11 00:51:35");
        assert_eq!(vec!["2001:0db8:0000:85a3:0000:0000:ac1f:8001"], v6(&text));
    }
}