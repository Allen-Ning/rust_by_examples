fn main() {
    let my_str : &'static str = "hello";
    let my_string : String = String::from(my_str);
    println!("{}", my_string);
}
