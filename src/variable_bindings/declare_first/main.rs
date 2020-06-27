fn main() {
    let var1;
    {
        let num = 2;
        var1 = num * num;
        println!("{}", var1);
    }
}