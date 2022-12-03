pub trait ToStrVec {
    /// .
    fn to_str_vec(&self) -> Vec<String>;
}

impl ToStrVec for &str {
    fn to_str_vec(&self) -> Vec<String> {
        self.trim().split('\n').map(Into::into).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_to_vec() {
        let input = "
first
second

third
        ";

        assert_eq!(input.to_str_vec(), vec!["first", "second", "", "third"])
    }
}
