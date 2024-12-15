fn main() {
    
    let mut value = 9;
    value = 10; //not allowed unless adding mut keyword to value declaration

    /*
        bindings are immutable by default, and this allows the program to run faster, with values which are known to be unchanging being kept in read only memory, and less checking has to be done on them
    */

    println!("the value binding holds {}", value);


    /*
        Besides using the mut keyword, there is another way to change a variable's value or type, called 'Shadowing', which is to run the let statement on it again:
    */

    let x = 9;

    // x = x + 1;       //not allowed

    let x = x + 1;      //allowed, using 'shadowing'

    let x = "Rust Language";    //allowed, using shadowing

    println!("The value of x is {}", x);
}
