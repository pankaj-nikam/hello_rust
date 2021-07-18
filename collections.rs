fn main() {
    //Arrays are similar in C#
    let sentence = ["This", "is", "a", "sentence"];
    //Arrays are also 0 based
    println!("{} {} {} {}", sentence[0], sentence[1], sentence[2], sentence[3]);
    //We can also print array using the following syntax
    println!("{:?}", sentence);
    //We can pretty print using the following syntax
    println!("{:#?}", sentence);
    //Array has len method to get the length of the array.
    println!("The length of the array is {}", sentence.len());

    //Like in C#, we have only one type of array. You cannot mix and match other type of elements.
    //let nameAge = ["Pankaj", 30];

    //Let us declare an array with single element.
    let _arr = ["p"];
    //Following code results into compilation failure because arrays are of fixed length and compiler knows that it contains only one element.
    //let _something_not_present = arr[1];

    //Runtime exceptions in Rust are called "panic".

    //Let us now see mutable arrays.
    //By default the arrays in Rust are read-only. That means you cannot set an element once its initialized.
    let _number_array = [1, 2, 3, 4];
    //If we try to change the element at 0th index, we get a compilation error. The compiler even suggests us to use mutable (mut) keyword.
    // number_array[0] = 3;

    //Now let us create a mutable array
    let mut fancy_number_array = [1, 2, 3, 4];
    fancy_number_array[0] = 0;
    println!("{:?}", fancy_number_array);

    //If you want to declare array with a length, you can declare as follows
    //This declares array of 5000 elements with 4.0 filled in all indices.
    let mut x = [4.00; 5000];
    x[2000] = 3.141516;
    println!("{} {}", x[1000], x[2000]);

    println!("Fibonacci Series:");
    //Let us see a fibonacci series with Rust
    let mut fib = [1; 15];
    for i in 2..fib.len() {
        fib[i] = fib[i - 2] + fib[i - 1];
    }
    println!("{:?}", fib);

    //Just as we have list in C#, we have vectors or vec in Rust which help us create dynamic collection
    let mut friends = vec!["Amit", "Snehal"];
    friends.push("Saurabh");
    println!("{:?}", friends);
}