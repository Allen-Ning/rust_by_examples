fn main() {
    let value = 5u8;
    let value2 = 7u8;
    let mut vec = Vec::new();
    vec.push(value);
    vec.push(value2);
    println!("{:?}", vec);
}