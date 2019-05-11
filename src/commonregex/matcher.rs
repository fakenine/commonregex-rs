extern crate regex;

use regex::Regex;

pub fn match_results<'a>(text: &'a String, regex: &'static str) -> Vec<&'a str> {
    let re = Regex::new(regex).unwrap();

    re.find_iter(text).map(|mat| mat.as_str()).collect()
}
