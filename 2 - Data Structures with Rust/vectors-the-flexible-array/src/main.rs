use std::collections::btree_map::Values;

fn main() {
    /*
        Vectors - The Flexible Array - Part 1

        Vec<T> is a commonly used data structure

        it's a grow/shrinkable arraylike that is allocated on the heap

    */

    let mut numbers: Vec<i32> = Vec::new();

    for i in 0..=9 {
        numbers.push(i);
    }

    println!("{numbers:?}");

    //creating a vector using the vector macro
    let mut letters: Vec<char> = vec![];

    for _ in 0..100 {
        letters.push('a');
    }

    println!("{letters:?}");

    //you can also initialize a vector this way:

    let mut this_vector = vec![1, 2, 3, 4, 5];

    //it seems like Vec::new() doesn't have an equivalent like being able to take in initial values as arguments

    // you can remove elements from a vector with vector.pop()

    //this method will return an Option with either Some(value_that_was_removed) or None if the vector was empty

    /*
        Vectors, - The Flexible Array Part 2
    */

    // you can index a vector but if the index is not valid the program will panic

    let vec_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("{}", vec_numbers[9]);
    // println!("{}", vec_numbers[12]);  //this panics


    //you can use vector.get() to safely check indexes and it has None if the index is invalid


    let i = 2;

    match vec_numbers.get(i) {
        Some(value) => println!("There was a value at index {i}, which is: {value}"),
        None => println!("There was no value at index {i}")
    }



    //using a for loop on a vector will consume it / move its values,
    //so use &vector references when iterating over one




    // you can preallocate capacity to a vector so that it doesn't have to reallocate dynamically as much in cases where the size you want is already known

    let mut preallocated_vector = Vec::with_capacity(10);

    for i in 0..11 {
        preallocated_vector.push(i);
    }

    //the capacity is 10 up until an 11th item is pushed on, and then the capacity doubles to 20

    println!("{:?}, capacity: {}", preallocated_vector, preallocated_vector.capacity());


}
