use anyhow::Result;

fn main() {
    let age: u8 = loop {
        match get_valid_age() {
            Ok(age) => break age,
            Err(e) => {
                println!("invalid input: {}", e)
            }
        }
    };
    println!("your age is: {}", age);
}

fn get_valid_age() -> Result<u8> {
    let s = get_input("What is your Age")?;
    let age: u8 = s.parse()?;
    Ok(age)
}
fn get_input(prompt: &str) -> Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let res: String = input.trim().to_owned();
    Ok(res)
}
