use std::collections::HashMap;
fn main() {

    /*
        HashMaps - part 1

        HashMap<K, V>

        - key value pairs

        - keys must be unique

        - key points to exactly one value

        - very fast lookup for data offering constant time complexity of 1,

            for insertion,

            deletion,

            and retrieval operations


            - to use it you have to bring it in with use std::collections::HashMap

     */

    let this_hashmap: HashMap<String, String> = std::collections::HashMap::new();


    // you can also let the types be inferred this way:

    let mut populationmap = HashMap::new();

    populationmap.insert("Tokyo", 27400100);
    populationmap.insert("London", 17400100);
    populationmap.insert("Cairo", 7400100);


    println!("{:#?}", populationmap);
    
    println!("{:#?}", populationmap["Tokyo"]);
    
    // println!("{:#?}", populationmap["hooberville"]);  //this caused a panic at runtime as the key didn't exist but there was no error at compile time about this
    

    //using hashmap.get(key) you have a some or a none, no panics if key is not present
    println!("{:#?}", populationmap.get("Tokyo"));
    println!("{:#?}", populationmap.get("Tokio"));

    let key_to_search = "London";
    match populationmap.get(key_to_search) {
        Some(value) => println!("{key_to_search} has : {value} population"),
        None => println!("{key_to_search} didn't appear in the hashmap")
    }

    let key_to_search = "Londo";
    match populationmap.get(key_to_search) {
        Some(value) => println!("{key_to_search} has : {value} population"),
        None => println!("{key_to_search} didn't appear in the hashmap")
    }
    


    //updating values in a hashmap

    // just insert again into an existing key

    populationmap.insert("London", 18000000);

    let key_to_search = "London";
    match populationmap.get(key_to_search) {
        Some(value) => println!("{key_to_search} has : {value} population"),
        None => println!("{key_to_search} didn't appear in the hashmap")
    }


    // deleting from a hashmap:

    populationmap.remove("London");

    let key_to_search = "London";
    match populationmap.get(key_to_search) {
        Some(value) => println!("{key_to_search} has : {value} population"),
        None => println!("{key_to_search} didn't appear in the hashmap")
    }


    //these seem to differ from a javascript object in that the valyes all have to be of the same data type

    // when I tried to insert a float it said 'expecting integer, found floating point number' so it isn't down to just a specific type like i8, i16, it seems to see the type as 'integer'





     /*
        HashMaps - part 2
     */



     //how to iterate over a hashmap


     //the for loop will consume / move the hashmap values if you don't use the reference borrowing here
     for (key, value) in &populationmap {
        println!("{key}: {value}");
     }

     for (city, population) in &populationmap {
        println!("{city}: {population}");
     }




     //get an array of just the values using .values() method
     
     // THEY WILL APPEAR IN ARBITRARY ORDER HOWEVER 


     println!("{:?}", populationmap.values());
     
     //get an array of the keys ysing hashmap.keys()
     println!("{:?}", populationmap.keys());



    // you can for loop it like this

    for value in populationmap.values() {
        println!("{}", value);
    }
    //it doesn't move the values when you use .values()
    for value in populationmap.values() {
        println!("{}", value);
    }


    // THIS IS A WAY TO CHECK IF AN ENTRY EXISTS, AND THEN IF IT DOES *NOT*, IT INSERTS A VALUE

    // this will insert delhi into the hashmap because it doesn't exist yet,
    // but tokyo will not be changed since it already existed

    populationmap.entry("Delhi").or_insert(30000000);
    populationmap.entry("Tokyo").or_insert(100000000);

    println!("{:?}", populationmap);



    // BETTER PERFORMANCE IN HASH MAPS:


    // the keys are hashed, and sometimes the hashes create collisions where they land 'in the same bucket'

    // so you can get better performance by using keys that are easy to hash efficiently, such as String, i32, usize


    // you can also preallocate space for the hash map

    let mut new_hashmap = HashMap::with_capacity(1);

    new_hashmap.insert(1, 1);

    println!("{:?} {} {}", new_hashmap, new_hashmap.capacity(), std::mem::size_of_val(&new_hashmap));
    
    new_hashmap.insert(2, 100000000);
    new_hashmap.insert(3, 100000000);
    new_hashmap.insert(4, 100000000);

    println!("(notice that the elements are not in the order in which they were inserted) {:?} {} {}", new_hashmap, new_hashmap.capacity(), std::mem::size_of_val(&new_hashmap));



    // when you preallocate 10 elements of space in the hash map, it might actually get 14

    // or when I got 1000 it showed 1732 or so

    // it has to do with the way the allocation algorithm works but it wasn't explained more than that

    // also, when iterating over the hashmap, the elements will not necessarily appear in the same order as they were inserted

    // this is because a hashmap is an unodered data structure

    // the order is determined by the hashing function and how the keys are distributed across the buckets

    // so the order they appear in reflects the order of how they're stored in memory


    // IF YOU WANT TO PRESERVE THE ORDER,

        // there is a 'Binary Tree Map'

        // and the 'index map'

}
