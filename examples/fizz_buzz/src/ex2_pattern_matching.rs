pub fn fizz_buzz(i: u32) -> String {
    let is_fizz = (i % 3) == 0;
    let is_buzz = (i % 5) == 0;

    let result = match (is_fizz, is_buzz) {
        (true, true) => String::from("FizzBuzz"),
        (true, _) => String::from("Fizz"),
        (_, true) => String::from("Buzz"),
        _ => i.to_string(),
    };

    result
}
