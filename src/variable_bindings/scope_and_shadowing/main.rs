fn main() {
    let one = 12i64;
    {
        let one = 20i128;
        println!("{:p}", &one);
    }

    let one = "abc";
    println!("{:p}", &one);
    println!("{:p}", &&one);


}