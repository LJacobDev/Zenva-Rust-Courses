fn main() {
    let mut s1 = String::from("Hello World");

    let mut s2 = s1;

    // println!("{s1}");    //s1 value borrowed here after move

    //move it back to s1

    s1 = s2;

    // println!("{s2}");    //s2 value borrowed here after move

    println!("{s1}");



    //ownership is how memory is managed, as there is no garbage collection

    //ownership rules:

    // each value in the code has an owner

    // there can be only one owner of each value at a time

    // when the owner goes out of scope, the value it owns is dropped / freed up from memory

    // no garbage collection is needed because of this, and it knows at COMPILE TIME when any used memory can be freed up instead of checking it at runtime

    
    let name = "Brinjo".to_string();
    
    print_name(&name);

    //functions can be defined inside of functions,
    //and they appear to be able to be defined under any line that invokes them
    fn print_name(name: &str) {
        println!("{name}");
    }

/* Slice Types */

let message = String::from("H🦀ello World");

//make a string slice by making a reference and using a range selection

//this takes elements 0, 1, 2, 3, 4 but not 5
//it is taking bytes and not characters,
//because inserting the 4 byte crab emoji in there makes 0..4 show
//only the "H🦀"
let slice_hello = &message[0..5];

println!("{slice_hello}");


//array slices:

let array = [1,2,3,4,5,6,7,8,9,10];

let array_slice = &array[0..3];

println!("{array_slice:?}");

}



