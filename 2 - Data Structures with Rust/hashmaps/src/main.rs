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

}
