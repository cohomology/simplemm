pub fn get_addresses_in_from_header(from_header: &str) -> std::vec::Vec<String> {
    lazy_static::lazy_static! {
        static ref REGEX : regex::Regex = regex::Regex::new("(?:[^<@\\s]*@[^\\s>,]*)|(?:\"[^\"@]*\"@[^\\s>,]*)").unwrap();
    }
    REGEX
        .find_iter(from_header)
        .map(|elem| elem.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_mails() {
        assert_eq!(
            super::get_addresses_in_from_header(
                "foo@bar, Frank Rotzelpü <frank@göckel.com>, Murkel <\"my murkel\"@localhost"
            ),
            std::vec!("foo@bar", "frank@göckel.com", "\"my murkel\"@localhost")
        );
    }
}
