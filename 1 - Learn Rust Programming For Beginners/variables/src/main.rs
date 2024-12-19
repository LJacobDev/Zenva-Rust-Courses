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

    /*
       Arrays can't change in size,
       they have homogeneous element types
       they have square brackets
       they are accessed using "array[0..n-1]"
    */

    // Compound variables - Tuples
    let person = ("Alice", 30, 5.4);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);

    /*
        tuples have round brackets
        tuples can have different types in the elements
        tuples are accessed using "tuple.0", "tuple.n-1"
    */

    // Constants
    const PI: f32 = 3.14;
    println!("Value of pi: {}", PI);

    /*
        constants conventionally are declared in all caps
        they can not be made mutable with mut keyword
        they can not be shadowed like how a binding can be
        and they must have a type specified, unlike where bindings can have the compiler infer its type
    */

    //string slice is a read only reference to a hard coded string literal

    let greeting: &str = "Hello world";
    println!("{}", greeting);

    //a string literal is implicitly assigned the &str type
    let name = "Frank";

    //owned Strings are mutable
    let mut name = String::from("Zenva");

    name.push(' ');
    name.push_str("Academy");

    println!("{}", name);

    //  Variables Challenge!

    /*  At every step, think:

    Whether it is best to use a mutable or immutable variable
    What would be the appropriate data type */

    /* 1. Store a person's name */

    let name = "Frank";

    /* 2. Use a variable to store the person's current age */

    let current_age: i8= 30;

    // in this one, they wanted age to be mutable based on the idea that a person's age changes over time (so this is more like a situation where this binding is linked to someone's profile that will change over time and not a static snapshot in time)

    /* 3. Use a constant for number of years */
    const NUMBER_YEARS: i8 = 10;

    /* 4. Calculate the person's age in the future */
    let future_age = current_age + NUMBER_YEARS;

    // in this one, they wanted 'age' to be mutable, and then to have it += operated on rather than storing it in a new binding

    /* 5. Print the name and the calculated future age */
    println!("{} will be {} in {} years", name, future_age, NUMBER_YEARS);
}
