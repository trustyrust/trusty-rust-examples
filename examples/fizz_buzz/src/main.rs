use anyhow::ensure;
use ex4_from_trait::FizzBuzz;

mod ex1_functional;
mod ex2_pattern_matching;
mod ex3_generic;
mod ex4_from_trait;

fn main() {
    // example functional programming
    println!("fizz_buzz for 30 is: {}", ex1_functional::fizz_buzz(30));

    // example pattern matching
    println!(
        "fizz_buzz for 30 is: {}",
        ex2_pattern_matching::fizz_buzz(30)
    );

    // example generics
    println!("fizz_buzz for 30 is: {}", ex3_generic::fizz_buzz(30_u64));

    // example from_trait using From
    println!("fizz_buzz for 30 is: {}", FizzBuzz::from(30).borrow_value());

    // example from_trait using into()
    let fizz_buzz: FizzBuzz = 30.into();
    println!("fizz_buzz for 30 is: {}", fizz_buzz.borrow_value());
}
