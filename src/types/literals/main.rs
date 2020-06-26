fn main () {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // i32
    let a = 1;
    // f64
    let b = 3.0;

    println!("{:?}", std::mem::size_of_val(&x));
    println!("{:?}", std::mem::size_of_val(&y));
    println!("{:?}", std::mem::size_of_val(&z));

    println!("{:?}", std::mem::size_of_val(&a));
    println!("{:?}", std::mem::size_of_val(&b));
}