fn main() {
    /*
       If Statement:

       if a number is divisible by 5 AND 3 print 'TriQuint'

       if a number is divisible by 6 AND 4, print 'HexaQuad'

       otherwise print 'Just another number'
    */

    for num in 0..100 {
        // for a number to be divisble by 5 and 3, it also needs to be divisible by 5 * 3
        if num % 15 == 0 {
            println!("{num} is a TriQuint");
        // for a number to be divisble by 6 and 4, it also needs to be divisible by 6 * 4
        } else if num % 20 == 0 {
            println!("{num} is a HexaQuad");
        } else {
            println!("Just another number");
        }
    }

    /*
        if let statement

        if is_weekend is true, assign 'go hiking' to the variable 'activity' otherwise assign 'no hiking'

    */

    let is_weekend = true;
    let activity = if is_weekend { "go hiking" } else { "no hiking" };

    println!("{activity}");

    let mut counter = 100;
    while counter > 0 {
        println!("{counter}");
        counter -= 1;
    }

    println!("while loop done");

    loop {
        //this will run indefinitely until a break statement happens
        //there is no need to make a loop like "while true {}"
        break;
    }





    //exercise:

    //generate fibonacci sequence looop

    let mut firstnumber = 0;
    let mut secondnumber = 1;
    
    print!("{firstnumber} ");
    print!("{secondnumber} ");

    for _ in 1..100 {
        let currentnumber: i128 = firstnumber + secondnumber;
        print!("{currentnumber} ");
        firstnumber = secondnumber;
        secondnumber = currentnumber;
    }




    //get average number from an array
    let temperatures = [10.0, 15.0, 12.8, 13.5, 18.0, 19.0, 22.1 ];

    let mut sum = 0.0;

    for temperature in temperatures {
        sum += temperature;
    }
    let average = sum / temperatures.len() as f64;
    println!("\n\n{average:.2}");

}



