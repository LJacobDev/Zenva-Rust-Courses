fn main() {
    let mut value = 9;
    value = 10; //not allowed unless adding mut keyword to value declaration

    /*
        bindings are immutable by default, and this allows the program to run faster, with values which are known to be unchanging being kept in read only memory, and less checking has to be done on them
    */

    println!("the value binding holds {}", value);

    // testing whether passing value to println! moves the value, and in the following lines it is seen that it does not, but then I also remembered that some primitive types like integers don't have their values moved when handed to functions

    println!("{}", value == 10);
    println!("{}", { value == 10 }); //both with and without curly braces is accepted
    println!("the value binding holds {}", value);

    /*
        Besides using the mut keyword, there is another way to change a variable's value or type, called 'Shadowing', which is to run the let statement on it again:
    */

    let x = 9;

    // x = x + 1;       //not allowed

    let x = x + 1; //allowed, using 'shadowing'

    let x = "Rust Language"; //allowed, using shadowing

    println!("The value of x is: {}", x);

    //copied and pasted these lines from the course, as it was nice to keep this as a reference but wasn't something that needed to be manually worked out:
    // Declaring scalar variables
    let small_value: i8 = 100; // 8-bit signed integer
    let sample_float: f32 = -700.25; // 32-bit float
    println!("Small int: {}", small_value);
    println!("Sample float: {}", sample_float);

    // Compound variables - Arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("Element at index 0: {}", numbers[0]);

    // Compound variables - Tuples
    let person = ("Alice", 30, 5.4);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);

    // Constants
    const PI: f32 = 3.14;
    println!("Value of pi: {}", PI);

    /*
        constants conventionally are declared in all caps
        they can not be made mutable with mut keyword
        they can not be shadowed like how a binding can be
        and they must have a type specified, unlike where bindings can have the compiler infer its type
    */
}
