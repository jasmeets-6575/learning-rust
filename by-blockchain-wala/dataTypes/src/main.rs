fn main() {
    let integar: i32 = 42;// Signed 32-bit integar
    let unsigned_integar: u64 = 10_000; // Unsigned 64-bit integar

    println!("Integar: {}", integar); 
    println!("Unsigned Integar: {}", unsigned_integar); 

    let float: f64 = 3.14; // 64-bit floating-point number
    println!("Float: {}", float);

    let is_true: bool = true;
    let is_false: bool = false; 

    println!("is True: {}", is_true);
    println!("is False: {}", is_false);
    
    let char1: char = 'A';
    println!("char is: {}", char1);
    
    let tup = (500, 6.4, 1);
    let (x, y,z ) = tup;
    println!("The vlaue of y is : {y}")
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

}
