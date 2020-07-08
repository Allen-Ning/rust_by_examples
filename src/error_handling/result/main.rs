use std::num::ParseIntError;

fn main() {
    let value = test("12345").unwrap();
    println!("this is the value : {}", value);
    let value2 = test("abc").unwrap();
    println!("this is what {}", value2);
}

fn test(value : &str) -> Result<i32, ParseIntError> {
    let num = match value.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    Ok(num)
}
