fn main() {
    let x = {
        let p = 101;
        println!("{}", p);
        1 + 15
    };
    let y = {
        let p = 102;
        println!("{}", p);
        1 + 15;
    };
    println!("{:?}", x);
    println!("{:?}", y);
}