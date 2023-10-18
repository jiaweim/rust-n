pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 以下断言的错误信息只包含给定表达式的返回值
        assert!(true);
        fn some_computation() -> bool { true }
        assert!(some_computation());
// 使用自定义报错信息
        let x = true;
        assert!(x, "x wasn't true!");
// 使用格式化的自定义报错信息
        let a = 3;
        let b = 27;
        assert!(a + b == 30, "a = {}, b = {}", a, b);
    }
}