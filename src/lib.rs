pub mod commonregex {
    mod regex_drawer;
    mod matcher;

    pub mod ip;
    pub mod phone;
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
        let text = String::from("On se rejoint au café ? Voici mon num 06 06 06 06 06");
        assert_eq!(vec!["06 06 06 06 06"], commonregex::phone::fr(&text));
    }

    #[test]
    fn phone_fr_dots() {
        let text = String::from("On se rejoint au café ? Voici mon num 06.06.06.06.06");
        assert_eq!(vec!["06.06.06.06.06"], commonregex::phone::fr(&text));
    }
}
