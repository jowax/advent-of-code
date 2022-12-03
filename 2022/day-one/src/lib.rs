pub fn first(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = first(2, 2);
        assert_eq!(result, 4);
    }
}
