fn main() {
    let mut data = (3.14, "Mathematics", '🥟');
    //You cannot do the following because tuples have fixed types inside it also.
    // data.0 = "Pankaj";
    data.1 = "Geometry";
    let copy_of_data = data;
    let empty_tuple = ();
    println!("{:?}", copy_of_data);
    println!("{:?}", empty_tuple);
}
