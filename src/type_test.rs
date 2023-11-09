#[cfg(test)]
mod type_test {
    #[test]
    fn as_test() {
        assert_eq!(10_i8 as u16, 10_u16); // 范围内转换
        assert_eq!(2525_u16 as i16, 2525_i16); // 范围内转换

        assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
        assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

        // 超出目标范围的转换生成的值等等与原始值对 2^N 取模的值
        // 其中 N 是按位算的目标宽度，有时也称为 "截断"
        assert_eq!(1000_i16 as u8, 232_u8);
        assert_eq!(65535_u32 as i16, -1_i16);

        assert_eq!(-1_i8 as u8, 255_u8);
        assert_eq!(255_u8 as i8, -1_i8);
    }

    #[test]
    fn checked_op() {
        // 10 与 20 之和可以表示为 u8
        assert_eq!(10_u8.checked_add(20), Some(30));

        // 100 和 200 之和不能表示为 u8
        assert_eq!(100_u8.checked_add(200), None);

        // 溢出
        assert_eq!((-128_i8).checked_div(-1), None);
    }

    #[test]
    fn wrappping_type() {
        // 第一个可以表示为 u16，第二个不能，所以得到 250000 对 2¹⁶ 的模
        assert_eq!(100_u16.wrapping_mul(200), 20000);
        assert_eq!(500_u16.wrapping_mul(500), 53392);

        // 有符号类型，可能回绕为负值
        assert_eq!(500_i16.wrapping_mul(500), -12144);

        // 在移位运算中，移位距离会在值的范围内回绕
        // 所以在 18 位类型上移动 17 位等价于移动 1 位
        assert_eq!(5_i16.wrapping_shl(17), 10);
    }

    #[test]
    fn float_test() {
        assert!((-1. / f32::INFINITY).is_sign_negative());
        assert_eq!(-f32::MIN, f32::MAX);
    }

    #[test]
    fn float_ops() {
        assert_eq!(5_f32.sqrt() * 5_f32.sqrt(), 5.);
        assert_eq!((-1.01_f64).floor(), -2.0);
    }

    #[test]
    fn bool_as() {
        assert_eq!(false as i32, 0);
        assert_eq!(true as i32, 1);
    }
}

#[cfg(test)]
mod tuple_test {
    #[test]
    fn test_split() {
        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");
    }
}

mod array_test {
    #[test]
    fn test_array() {
        let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
        let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

        assert_eq!(lazy_caterer[3], 7);
        assert_eq!(taxonomy.len(), 3);
    }

    #[test]
    fn test_sort() {
        let mut chaos = [3, 5, 4, 1, 2];
        chaos.sort();
        assert_eq!(chaos, [1, 2, 3, 4, 5]);
    }
}

