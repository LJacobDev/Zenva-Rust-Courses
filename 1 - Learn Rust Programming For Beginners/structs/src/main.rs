
// structs encapsulate related data into one complex type so that you can track it with one binding handle rather than trying to manage it in several bindings
// structs have named fields with types assigned to them
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}




//There are also Tuple Structs, which can be used when the values don't need labels to make sense,
//like with RGB colour values or XYZ location points

#[derive(Debug)]
struct Colour (u8, u8, u8);

#[derive(Debug)]
struct Point (f64, f64, f64);



fn main() {

    let user = User {
        name: String::from("Alice"),
        email: "alice@example.com".to_string(),
        active: true,
        sign_in_count: 5
    };


    println!("{} {}", user.name, user.email);

    fn print_user(user: &User) {
        println!("hello {:#?}", user);
    }


    print_user(&user);
    //you don't seem to be able to access it like user["email"], an error message appears then

    let colour = Colour(255, 255, 255);

    let point = Point(0.0,0.0,0.0);

    println!("{colour:?} {point:?}");

    println!("{:?}", colour.1);

}



