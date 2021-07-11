fn main() {
    //These are actually constants.
    let first_number = 13;
    let second_number = 25;
    println!("The addition of {} and {} is {}", first_number, second_number, first_number + second_number);
    //second_number = 30; // We cannot do this since the second_number is not mutable.
    
    //This is a mutable variable. This is different from C# where we have all variables mutable.
    let mut third_number = 500;
    println!("Before {}", third_number);
    //Updating the value.
    third_number = 510;
    println!("After {}", third_number);

    //If the variable does not require to be mutable, Rust compiler generates a warning.
    // let mut mutable_but_not_required = 30;
    let mutable_but_not_required = 30;
    println!("Simply displaying the value: {}", mutable_but_not_required);

    //If we declare a variable but do not initialize it, it results into compiler error. Same in C#.
    //Code below results in compiler error:
    
    // let number_declared : i32;
    // println!("{}", number_declared);

    //When we have unused variables in code, it generates warnings like in C#.
    let unused_variable = 59;

    //If we have to disable the warning, then we can prepend _ (underscore) to the variable name like:
    let _another_unused_variable = 5555;

    //Like discards in C#, we also have discards in Rust. Same syntax with only _ (underscore).
    let _ = 8989;

    //We can also declare multiple discards.
    let _ = 999;
}