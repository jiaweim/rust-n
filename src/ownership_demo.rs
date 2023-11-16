#[cfg(test)]
mod move_owner {
    use std::rc::Rc;

    #[test]
    fn vec_index_string() {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        let fifth = v.pop().expect("vector empty");
        assert_eq!(fifth, "105");

        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        let third = std::mem::replace(&mut v[2], "substitute".to_string());
        assert_eq!(third, "103");

        assert_eq!(v, vec!["101", "104", "substitute"])
    }

    #[test]
    fn no_trace_type() {
        struct Person {
            name: Option<String>,
            birth: i32,
        }

        let mut composers = Vec::new();
        composers.push(Person {
            name: Some("Palestrina".to_string()),
            birth: 1525,
        });


        let first_name = std::mem::replace(&mut composers[0].name, None);
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }

    #[test]
    fn rc_demo() {
        use std::rc::Rc;

        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t: Rc<String> = s.clone();
        let u: Rc<String> = s.clone();

        assert!(s.contains("shira"));
        assert_eq!(t.find("taki"), Some(5));
        println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    }
}

#[cfg(test)]
mod refer_demo {
    #[test]
    fn triple_ref() {
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 1000, y: 769 };
        let r = &point;
        let rr = &r;
        let rrr = &rr;

        assert_eq!(rrr.y, 769);
    }

    #[test]
    fn compare_ref() {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);
    }
}