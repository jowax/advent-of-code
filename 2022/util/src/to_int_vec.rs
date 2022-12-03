pub trait ToIntVec {
    fn to_int_vec(self) -> Vec<i32>;
}

impl ToIntVec for Vec<String> {
    fn to_int_vec(self) -> Vec<i32> {
        self.into_iter().flat_map(|it| it.parse().ok()).collect()
    }
}
