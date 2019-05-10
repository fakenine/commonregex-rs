extern crate regex;

pub mod commonregex {
    use regex::Regex;

    const IPV4: &'static str = r"(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)";

    fn match_results<'a>(text: &'a str, regex: &'static str) -> Vec<&'a str> {
        let re = Regex::new(regex).unwrap();

        re.find_iter(text).map(|mat| mat.as_str()).collect()
    }

    pub fn ipv4(text: &str) -> Vec<&str> {
        match_results(text, IPV4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_ok() {
        let text = "Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35";
        assert_eq!(vec!["127.0.0.1"], commonregex::ipv4(text));
    }
}
