extern crate regex;

mod regex_drawer;

pub mod commonregex {
    use regex::Regex;
    use super::regex_drawer;

    fn match_results<'a>(text: &'a str, regex: &'static str) -> Vec<&'a str> {
        let re = Regex::new(regex).unwrap();

        re.find_iter(text).map(|mat| mat.as_str()).collect()
    }

    pub mod ip {
        use super::*;

        pub fn v4(text: &str) -> Vec<&str> {
            match_results(text, regex_drawer::ip::V4)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4() {
        let text = "Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35";
        assert_eq!(vec!["127.0.0.1"], commonregex::ip::v4(text));
    }
}
