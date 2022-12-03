use crate::to_str_vec::ToStrVec;

pub trait GroupedByDoubleNewline {
    fn to_grouped_by_double_newline(&self) -> Vec<Vec<String>>;
}

impl GroupedByDoubleNewline for &str {
    fn to_grouped_by_double_newline(&self) -> Vec<Vec<String>> {
        self.split("\n\n")
            .into_iter()
            .map(|item| item.to_str_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_group() {
        let input = "
first
second

third
        ";

        assert_eq!(
            input.to_grouped_by_double_newline(),
            vec![vec!["first", "second"], vec!["third"]]
        )
    }
}
