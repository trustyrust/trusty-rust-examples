pub fn fizz_buzz(i: u32) -> String {
    let mut str = String::new();

    if i % 3 == 0 {
        str += "fizz";
    }
    if i % 5 == 0 {
        str += "buzz";
    }
    if str == "" {
        str = i.to_string();
    }

    str.to_string()
}
