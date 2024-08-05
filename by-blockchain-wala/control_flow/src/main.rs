use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let mut x = 0;
    loop {
        println!("Again");
        x += 1 ;
        println!("value of x == {x}");

        if x >= 5 {
            break;
        }

        thread::sleep(time::Duration::from_secs(2));

    }

    while x<5 {
        x+=1;
        println!("again!");
        println!("value of x == {x}");
        thread::sleep(time::Duration::from_secs(2));
    } 

    for element in a {
        println!("the value is : {element}");
        thread::sleep(time::Duration::from_secs(1));
    }

    for number in (1..4).rev() {
        println!("the number is : {number}");
        thread::sleep(time::Duration::from_secs(1));    }

}
