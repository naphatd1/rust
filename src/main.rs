use std::{fmt::format, string};

fn main() {
    // let mut crabby_energy = 100;
    // crabby_energy = 90;
    // println!("Crabby energy : {}", crabby_energy)
    let x = 1;
    let y = 0.5;

    let z = x + y as i32;
    let msg: String = String::from("hello world");
    let msg2 = "Hello World".to_string();
    let msg3: &str = "Hello World";
    let msg4: String = format!("Point: {}, {}", x, y);
    println!("{}", msg4)
}
