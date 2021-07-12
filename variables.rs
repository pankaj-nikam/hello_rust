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

    //Boolean in Rust is similar to that of C#.
    let truth = true;
    let falsity = false;

    println!("{} {}", truth, falsity);

    //We can evaluate expressions as we do in C#.

    let is_three_greater_than_two = 3 > 2;
    let is_four_smaller_than_five = 4 < 5;
    println!("Is Three Greater Than Two? {}", is_three_greater_than_two);
    println!("Is Four Smaller Than Five? {}", is_four_smaller_than_five);

    //Since we are talking about booleans, it is same as in C#, && is logical AND, || is logical OR ! is NOT

    //Like var in C#, when we use let keyword to declare and initialize a variable, we have to stick to the type we initially declared.

    let mut some_number = 1;
    println!("{}", some_number);
    some_number = 2;
    // some_number = 33.33; //This results in error because when we declared it we assigned it to integer.

    //We can assign some other number to the some number variable because its type is already declared.
    //Yes the underscore prefix means that we wont be using the some_other_number variable.
    let _some_other_number = some_number;

    //Now dont get freaked out by the following line:
    let some_number = 3.14

    //Did you see what just happened above? Yes, we "re-declared" some_number. That is serious error in C#, however rust allows it.
    //It is allowed because it is not overwritten however it is shadowed by the new one.

    //Arithmetic operations are same as in C#. So no need to do it again here.
    //Abbreviated forms are also allowed (+=, -=, *=, /=)

    
}