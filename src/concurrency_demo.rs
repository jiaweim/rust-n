use rayon::prelude::*;

#[cfg(test)]
mod rayon_demo {
    #[test]
    fn flat_map_test() {
        let a = [[1, 2], [3, 4], [5, 6], [7, 8]];

        let par_iter = a.par_iter().cloned().flat_map(|a| a.to_vec());

        let vec: Vec<_> = par_iter.collect();

        assert_eq!(&vec[..], &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn demo() {
        use std::collections::HashSet;
        let squares = [4, 9, 16, 25, 36, 49, 64];
        let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>)
            = squares.iter().partition(|&n| n & (n - 1) == 0);

        assert_eq!(powers_of_two.len(), 3);
        assert_eq!(impure.len(), 4);
    }

    #[test]
    fn partition() {
        let (upper, lower): (String, String)
            = "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
        assert_eq!(upper, "GTO");
        assert_eq!(lower, "reat eacher nizuka");

        let params = glium::DrawParameters {
            line_width: Some(0.02),
            point_size: Some(0.02),
            ..Default::default()
        };

        target.draw(..., &params).unwrap();
    }
}
