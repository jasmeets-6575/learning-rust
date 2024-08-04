fn main() {
    println!("Hello world");
    let sum1 = sum_of_xandy(30, 50);
    another_function();
    println!("Sum of x and y is {sum1}");
}

fn sum_of_xandy(x: u32, y:u32) -> u32 {
    let sum = x + y;
    // return sum;
    sum
}

fn another_function() {
    println!("Another function");
}
