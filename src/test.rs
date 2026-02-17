use std::io::{self, Write};

pub fn manual_test(tester_ask: &str, name: &str){
    let mut resault = true;

    println!("\n {} [Y/n]: ", tester_ask);
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer = answer.trim().to_lowercase();

    if answer == "n" {resault = false};
    assert_eq!(resault, true, "Тест {} не пройден", name);
}
