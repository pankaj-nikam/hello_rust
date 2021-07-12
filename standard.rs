fn main() {

    let first_name = "Pankaj";
    let last_name = "Nikam";

    //Using standard library to get length and calling method over string to get length.
    println!("Length of {} is {}. Length of {} is {}.", first_name, str::len(first_name), last_name, last_name.len());
}