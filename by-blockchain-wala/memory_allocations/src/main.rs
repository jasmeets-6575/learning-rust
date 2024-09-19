static mut COUNT: u32 = 0 ;
const PI: f32 = 3.14159;

fn main() {
    println!("Hello, world!");

    // Accessing static variable 
    unsafe {
        println!("Initial Count: {}", COUNT);
        COUNT += 1;
        println!("Updated copunt: {}", COUNT);
    }

    // Calling a function that modifies the static variable
    increment_count();
    // Accessing static variable again 
    unsafe {
        println!("Final Count: {}", COUNT);
    }
    println!("Area of circle with radius of 10 is {}", area_of_circle(10.0));
}

fn increment_count(){
    unsafe {
        COUNT += 1;
    }
}

fn area_of_circle(radius: f32) -> f:32 {
    PI * radius * radius;
}