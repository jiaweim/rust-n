// use std::convert::TryInto;

use std::fmt::Display;

fn main() {
    let s = "hello";
    println!("{}, world", s);

    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");
}
