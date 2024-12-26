mod calculations;
use num::integer::gcd_lcm;
fn main() {
    let income = 45000.0;
    let tax = calculations::calculate_income_tax(income);
    println!("Annual Income Tax: ${:.2}", tax);
    let principal = 1000.0;
    let rate = 0.05;
    let time = 10.0;
    let n = 4.0;
    let compound_amount = calculations::calculate_compound_interest(principal, rate, time, n);
    println!("Compound Interest Amount: ${:.2}", compound_amount);
    let simple_amount = calculations::calculate_simple_interest(principal, rate, time);
    println!("Simple Interest Amount: ${:.2}", simple_amount);
    let tax_rate = calculations::get_tax_rate(income);
    println!("Tax Rate: {:.2}%", tax_rate * 100.0);

    let x = 30;
    let y = 234100;
    println!("GCD and LCM of {} and {} are: {:?}", x,y, gcd_lcm(x, y));
}


/*
    in the instructor's version they made two modules, tax.rs and interest.rs
*/


/*
    for challenge #2, they want to add something to the code in challenge 1, so I think I won't make a challenge 2 project after all and will just add to this one

    Challenge 2:

    "install the num crate and use it to calculate the greatest common divisor (GCD) and the least common multiple (LCM) of two numbers.  Add this to the code from the previous challenge."

    I've added it by looking up on crates.io and then docs.rs about num crate and any functions that were about gcd, and I found one that returns a tuple of both gcd and lcm
    
*/