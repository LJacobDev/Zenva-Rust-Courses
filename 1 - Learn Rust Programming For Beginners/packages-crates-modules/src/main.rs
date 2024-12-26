use rand::Rng;

fn main() {
    /*

       'A crate is the smallest amount of code that the Rust compiler considers at a time'

       Types of crate:

           Binary - has a main() function

           Library - doesn't have a main() function but can have its contents imported and used by other crates


       Dependency:

           dependencies provide pre-written functionality

           developers can use them rather than writing everything from scratch

           dependencies are often library crates

           these can be downloaded and managed by cargo



       Example:

           if you want to generate a random number,

           you can either implement your own random number generator,

           or you can get a dependency or a package that was made by another developer like the Rand crate


           in the CLI run "cargo add rand"

           or you can also add 'rand = "0.1.0"' in cargo.toml under [dependencies]

           and using an extension like Dependi you can see if there is a newer version available right in the cargo.toml file.  I put in rand = "0.1.0" and dependi showed to the right of my written text that 0.8.5 is the current available version


       TO LEARN HOW TO USE A CRATE'S CONTENTS:

           there will be documentation to go with the crate that explains its usage

    */

    //this creates a random number generator
    let mut rng = rand::thread_rng();
    //  println!("{:#?}", rng);

    for _ in 0..10 {
        //this makes the random number generator produce a random number
        let random_number = rng.gen_range(0..=100);

        println!("{}", random_number);
    }

    /*
       MODULES:


       if your main.rs has these four functions:

       greet()

       farewell()

       add()

       subtract()


       it could be better to put them into modules grouped by purpose


       modules can be private, so that they are not available outside of the crate,

       or public, which can expose the module to external code


       so the greeting and farewell functions can go into messages.rs,

       and the add / subtract ones can go in calculations.rs

       they just need 'pub' put in front of each function signature to make them able to be used

       then, in this file, add 'mod messages' and 'mod calculations' to enable using them

       then call the functions like messages::greet(), calculations::add()

    */


    println!("{}",messages::greet("Keven"));
    println!("{}",calculations::add(1,2));
}
mod messages;
mod calculations;
//it is possible to put mod statements anywhere in the global scope apparently, don't have to be in the top of the file
