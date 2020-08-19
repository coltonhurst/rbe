// https://doc.rust-lang.org/stable/rust-by-example/conversion/from_into.html

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // From
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Into
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
