fn main() {
    println!("hello");
    pass_my_bless("what is this");
    pass_my_bless("shit");
}

fn pass_my_bless(sentence: &str) {
    if sentence == "shit" { panic!("this is the bad word"); }
    println!("{}", sentence);
}
