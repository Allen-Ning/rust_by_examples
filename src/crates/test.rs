pub fn test1() {
    println!("test1");
}

pub fn test2() {
    println!("test2");
    private_test3();
}

fn private_test3() {
    println!("test3");
}