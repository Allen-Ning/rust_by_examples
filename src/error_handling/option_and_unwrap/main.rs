fn check(option: Option<&str>) {
    match option {
        Some("first") => println!("haha {:#?}", option),
        Some("second") => println!("wowo {:#?}", option),
        Some(everything) => println!("all the rest {}", everything),
        None => println!("no option"),
    }
}

fn check2(option: Option<&str>) {
   let value : &str = option.unwrap();
   if value == "shit" { panic!("this is - {} bad word", value); }
   println!("good value {}", value);
}

fn main () {
    let first = Some("first");
    let second = Some("second");
    let others = Some("others");
    let none = None;
    check(first);
    check(second);
    check(others);
    check(none);

    let four = Some("four");
    let five = Some("five");
    check2(four);
    check2(five);
    check2(None);
}