pub struct FizzBuzz {
    val: String,
}
impl FizzBuzz {
    pub fn borrow_value(&self) -> &str {
        &self.val
    }
    pub fn take_value(self) -> String {
        self.val
    }
}
impl From<u32> for FizzBuzz {
    fn from(i: u32) -> Self {
        let is_fizz = (i % 3) == 0;
        let is_buzz = (i % 5) == 0;

        let result = match (is_fizz, is_buzz) {
            (true, true) => String::from("FizzBuzz"),
            (true, _) => String::from("Fizz"),
            (_, true) => String::from("Buzz"),
            _ => i.to_string(),
        };

        FizzBuzz { val: result }
    }
}
