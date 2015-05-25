extern crate fizzybuzz;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    println!("Fizz Buzz!");

    for x in 1..25 {
      println!("{}: {}", x, fizzybuzz::word_for(x)); // x: i32
  }
}
