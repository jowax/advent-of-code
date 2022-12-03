pub trait ToGroupedIntVec {
    fn to_grouped_int_vec(self) -> Vec<Vec<i32>>;
}

impl ToGroupedIntVec for Vec<Vec<String>> {
    fn to_grouped_int_vec(self) -> Vec<Vec<i32>> {
        self.into_iter()
            .map(crate::to_int_vec::ToIntVec::to_int_vec)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        to_grouped_by_double_newline::GroupedByDoubleNewline, to_grouped_int_vec::ToGroupedIntVec,
    };

    #[test]
    fn multi_int_group() {
        let input = "
1
2

3
        ";

        assert_eq!(
            input.to_grouped_by_double_newline().to_grouped_int_vec(),
            vec![vec![1, 2], vec![3]]
        )
    }
}
