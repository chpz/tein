extern crate tein;

use tein::{Tein, Inputer};

fn main() {
    let stdin = std::io::stdin();
    let mut t = Tein::new(stdin.lock());
    let count: usize = t.next().unwrap();
    let nums: Vec<i32> = t.iter().take(count).collect();
    println!("{:?}", nums);
}
