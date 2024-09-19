static mut COUNT: u32 = 0;

const PI: f32 = 3.14159;

fn main () {
    unsafe {
        println!("Count: {}", COUNT);
        COUNT + 1;
        println!("Updated Count: {}", COUNT);
    }
    increment_count();

    unsafe {
        println!("Final count: {}", COUNT);
    }

    println!("area of the circle with radius 10 is ");
    
}

fn increment_count(){
    unsafe {
        COUNT += 1;
    }
}

fn area_of_circle(radius: f32) -> f32 {
    PI * radius * radius
}
