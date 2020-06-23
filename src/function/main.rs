fn main() {
    test_range(10);
    println!("{}", "test closure+++++++++++++++++++++++++");
    test_closure();
}

fn test_range(value: u32) {
    for n in 0..value + 10 {
        println!("{}", n);
    }
}

fn test_closure() {
    let one = || 1;
    println!("{}", one());

    let two = |i: u32| -> u32 { i + 1 };
    println!("{}", two(100));

    let three = |i| {i + 1};
    println!("{}", three(200));

}
