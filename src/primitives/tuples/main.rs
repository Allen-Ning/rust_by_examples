fn main() {
    let tuples = ("123", 456, 12.34);
    let (first, second, third) = tuples;
    println!("first: {}", first);
    println!("second: {}", second);
    println!("third: {}", third);

    let pair = reverse_pair((23, false));
    println!("{:?}", pair);

    let tuples_of_tuples = (123, (456, "haha", 47i128));
    println!("nested tupples {:?}", tuples_of_tuples);
}

fn reverse_pair(pair: (i32, bool)) -> (bool, i32) {
    let (num, boolean) = pair;
    return (boolean, num);
}