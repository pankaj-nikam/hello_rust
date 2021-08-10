fn main() {
    let hexadecimal = 0x10;
    let decimal = 10;
    let octal = 0o10;
    let binary = 0b10;
    println!("{} {} {} {}", hexadecimal, decimal, octal, binary);
    let one_crore = 1_00_00_000;
    println!("One crore: {}", one_crore);
    let scientific_notation = 1e8;
    println!("Scientific notation: {}", scientific_notation);
    let thirteen_millionths = 13e-6;
    println!("Thirteen Millionths {}", thirteen_millionths);

    //In rust we have ten integer types and two floating types
    //Let us see how they are declared and used
    //What we declared below are signed numbers. So they store the values with their sign (positive or negative)
    let eight_bit: i8 = 5; //Stores between -128 and 127 (including both)
    let sixteen_bit: i16 = 5;
    let thirty_two_bit: i32 = 5;
    let sixty_four_bit: i64 = 5;
    println!(
        "Eight Bit: {} | Sixteen Bit: {} | Thirty Two Bit: {} | Sixty Four Bit: {}",
        eight_bit, sixteen_bit, thirty_two_bit, sixty_four_bit
    );

    //To perform arithmetic operations, the numbers should be of same types.
    //Code below results in compilation error

    // let sum = eight_bit + sixteen_bit;
    // println!("Sum is {}", sum);

    //Let us see some unsigned integer values
    let unsigned_eight_bit: u8 = 5;
    let unsigned_sixteen_bit: u16 = 5;
    let unsigned_thirty_two_bit: u32 = 5;
    let unsigned_sixty_four_bit: u64 = 5;

    println!("Unsigned values:");
    print!(
        "8: {} | 16: {} | 32: {} | 64: {}",
        unsigned_eight_bit, unsigned_sixteen_bit, unsigned_thirty_two_bit, unsigned_sixty_four_bit,
    );

    //Let us see floating point values
    println!("Floating Values:");
    let float_64: f64 = 3.14;
    let float_32: f32 = 3.14;
    println!("32 Bit: {} | 64 Bit: {}", float_64, float_32);

    //In C#, we can put smaller variable in bigger type, however in rust we cannot do it implicitly.
    //Following lines result into compiler error.

    // let mut some_variable: f64 = 1.0;
    // some_variable = float_32;

    //If we do want to set the variable, we can use the following syntax which converts it explicitly
    //We have to use the 'as' keyword to convert it to specific type.
    let mut some_variable: f64 = 1.0;
    println!("Old value {}", some_variable);
    some_variable = float_32 as f64;
    println!("New value {}", some_variable);

    //Rust does not overflow the values if they are out of the bounds.
    //It results into compile time error if the compiler notices that the value will be out of bound.
    //However if the values change at runtime, it results into runtime exception.
    //It overflows in release mode and not compile mode.
    //Example:
    // let mut something_big: i8 = 1;
    // something_big += 127;
    // println!("The value of something big is {}", something_big);

    //Rust also has primitives like char and boolean as in C#
    let first_character = 'a';
    let is_sun_bigger_than_moon = true;
    println!("The first character is '{}'", first_character);
    println!("Is sun bigger than moon? {}", is_sun_bigger_than_moon);

    //Rust supports unicode characters
    let mu = 'µ';
    println!("Mu is {}", mu);

    //Let us loop over the numbers
    for i in 0..256 {
        println!("{}: [{}]", i, i as u8 as char);
    }

    //There is no "void" type in Rust. The Empty Tuple indicates it is void.
    let a = ();
    println!("The void type: {:?}", a);

    //Let us learn how to declare arrays with explicit type:
    let my_int_array: [i32; 2] = [0, 1];
    println!("The array contains {:?}", my_int_array);

    //Similarly, we can declare vectors with explicit type:
    let float_values: Vec<f32> = vec![3.14, 22.0, 7.0];
    println!("The float vector contains {:?}", float_values);

    //The array size must be known at compile time. We cannot declare a variable using let and assign array the size.
    //However we can declare a constant using const keyword.
    const N: usize = 20;
    let _something = [0; N];
    println!("The compile time array contains {:?}", _something);
}
