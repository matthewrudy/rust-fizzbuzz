extern crate fizzybuzz;

fn main() {
    println!("Fizz Buzz!");

    for x in 1..25 {
      println!("|{}|", fizzybuzz::word_for(x)); // x: i32
  }
}
