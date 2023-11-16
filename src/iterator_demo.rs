#[cfg(test)]
mod it_demo {
    fn triangle(n: i32) -> i32 {
        (1..=n).fold(0, |sum, item| sum + item)
    }

    #[test]
    fn fold_demo() {
        let a = triangle(3);
        assert_eq!(a, 6);
    }
}