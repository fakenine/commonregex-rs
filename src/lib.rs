extern crate regex;

mod regex_drawer;

pub mod commonregex {
    use regex::Regex;
    use super::regex_drawer;

    fn match_results<'a>(text: &'a String, regex: &'static str) -> Vec<&'a str> {
        let re = Regex::new(regex).unwrap();

        re.find_iter(text).map(|mat| mat.as_str()).collect()
    }

    pub mod ip {
        use super::*;

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
        /// let log = String::from("Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35");
        ///
        /// assert_eq!(vec!["127.0.0.1"], commonregex::ip::v4(&log));
        /// ```
        pub fn v4(text: &String) -> Vec<&str> {
            match_results(text, regex_drawer::ip::V4)
        }
    }

    pub mod phone {
        use super::*;

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
        /// assert_eq!(vec!["127.0.0.1"], commonregex::phone::fr(&text));
        /// ```
        pub fn fr(text: &String) -> Vec<&str> {
            match_results(text, regex_drawer::phone::FR)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4() {
        let text = String::from("Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35");
        assert_eq!(vec!["127.0.0.1"], commonregex::ip::v4(&text));
    }

    #[test]
    fn phone_fr() {
        let text = String::from("On se rejoint au café ? Voici mon num 0606060606");
        assert_eq!(vec!["0606060606"], commonregex::phone::fr(&text));
    }

    #[test]
    fn phone_fr_spaces() {
        let text = String::from("On se rejoint au café ? Voici mon num 0606060606");
        assert_eq!(vec!["06 06 06 06 06"], commonregex::phone::fr(&text));
    }

    #[test]
    fn phone_fr_dots() {
        let text = String::from("On se rejoint au café ? Voici mon num 0606060606");
        assert_eq!(vec!["06.06.06.06.06"], commonregex::phone::fr(&text));
    }
}
