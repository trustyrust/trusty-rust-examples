use num::{PrimInt, Unsigned};

pub fn fizz_buzz<T>(i: T) -> String
where
    T: PrimInt + Unsigned,
    T: std::fmt::Display,
{
    let zero = T::zero();
    // using unwrap because 3 and 5 will never fail being an unsigned int
    let is_fizz = (i % T::from(3).unwrap()) == zero;
    let is_buzz = (i % T::from(5).unwrap()) == zero;

    let result: String = match (is_fizz, is_buzz) {
        (true, true) => "FizzBuzz".to_string(),
        (true, _) => "Fizz".to_string(),
        (_, true) => "Buzz".to_string(),
        _ => {
            // safe to unwrap becasue i is defined as T in the variable definition
            T::from(i).unwrap().to_string()
        }
    };

    result
}
