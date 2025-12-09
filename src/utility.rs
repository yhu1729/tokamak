#[macro_export]
macro_rules! assert_eq_within {
    ($left:expr, $right:expr, $tolerance:expr) => {
        match (&$left, &$right, &$tolerance) {
            (x, y, t) => {
                assert!((*x - *y).abs() < *t)
            }
        }
    };
}
