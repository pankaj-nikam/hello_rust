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

    //Following code results into compilation failure because arrays are of fixed length and compiler knows that it contains only one element.
    let arr = ["p"];
    let _something_not_present = arr[1];

    //Runtime exceptions in Rust are called "panic".

    
}