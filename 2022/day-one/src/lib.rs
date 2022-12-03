use util::{GroupedByDoubleNewline, ToGroupedIntVec};

/// https://adventofcode.com/2022/day/1
pub fn first_task(input: &str) -> i32 {
    let list = input.to_grouped_by_double_newline().to_grouped_int_vec();

    let mut entries = list
        .into_iter()
        .map(|it| it.iter().sum::<i32>())
        .collect::<Vec<_>>();

    entries.sort();
    entries.reverse();

    *entries.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first_task(include_str!("first_test_input.txt")), 24000);
    }

    #[test]
    fn first() {
        println!("First result: {:#?}", first_task(include_str!("first_input.txt")));
    }
}
