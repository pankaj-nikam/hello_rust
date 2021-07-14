fn main() {
    let number = 0;
    if number > 0 { 
        println!("{} is Positive", number);
    }
    else if number < 0 {
        println!("{} is Negative", number);
    }
    else {
        println!("{} is Neither negative, nor positive.", number);
    }

    //There is a while statement in rust but not a "do-while" statement.
    let mut i = 1;
    while i <= 20 {
        println!("{} squared is {}",i, i * i);
        i += 1;
    }

    //We can modify the loop prematurely using break and continue statement.
    //The loop below shows square of odd numbers only.
    let mut i = 0;
    while i <= 200 {
        i += 1;
        if i % 2 == 0 { continue; }
        println!("{} squared is {}",i, i * i);
    }

    //In C# we often to forever loops (Infinite Loops) using "while true" statement, it is valid in rust too.
    //However rust provides a clean approach to it using the following syntax:

    let mut i = 1;
    loop {
        let j = i * i;
        if j >= 400 { break; }
        println!("{}", j);
        i += 1;
    }

    //Let us see the for loop in Rust
    println!("For Loop");
    for i in 1..11 {
        println!("{}", i * i);
    }

    //Following is an interesting case of shadowing

    println!("Shadowing:");

    let index = 8;

    for index in 0..4 {
        println!("{}", index);
    }

    println!("{}", index);

    //Now guess the output of the following program:
    let mut limit = 5;

    for i in 1..limit {
        limit -= 1;
        println!("{}", i);
    }
    println!("{}", limit);
}