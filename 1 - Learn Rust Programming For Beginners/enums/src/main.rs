use std::{default, result, thread::park};

//enums can be used to give a type a specific number of predefined states
//which is safer and easier to work with than relying on it being given a value that is
//stored as a string, which would be prone to typing errors and other mistakes and difficulties
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let traffic_light = TrafficLight::Green;

    // the following code wasn't accepted, it wasn't allowing TrafficLight to have == used
    // so it needs a match statement kind of approach
    // if traffic_light == TrafficLight::Green {
    //     println!("Go");
    // } else if traffic_light == TrafficLight::Yellow {
    //     println!("Stop if you can");
    // } else {
    //     println!("Stop");
    // }

    match traffic_light {
        TrafficLight::Green => println!("Go"),
        TrafficLight::Yellow => println!("Stop if you can"),
        TrafficLight::Red => println!("Stop"),
    }

    //this is an alternative to the match statement,
    //it does not seem to require that all possible variants are checked for
    //which would allow doing special actions only in cases where they're needed
    //and not having to write 'else' actions for other cases
    if let TrafficLight::Green = traffic_light {
        println!("Traffic light is green");
    }

    //Enums that hold values:
    enum Shape {
        Circle(f32),         //radius
        Rectangle(f32, f32), //length and width
        Square(f32),         //only need length of one side
    }

    fn calculate_area(shape: Shape) {
        match shape {
            Shape::Circle(radius) => println!("area of circle is {}", 3.14 * radius * radius),
            Shape::Rectangle(length, witdh) => println!("Area of rectangle is {}", length * witdh),
            Shape::Square(side) => println!("Area of square is {}", side * side),
        }
    }

    let circle = Shape::Circle(3.0);
    let rectangle = Shape::Rectangle(10.0, 15.0);
    let square = Shape::Square(20.0);

    calculate_area(circle);
    calculate_area(rectangle);
    calculate_area(square);

    /*
       The Option Enum

       enum Option {
           None,
           Some(T)
       }

       useful in instances where there is the possibility of no data,

       like searching for a username in a database but the username is not in the database

       if the username isn't in the database, it gives 'None' option and you handle that the username wasn't present

       if the username is in the database, it gives Some(User) and you can do things with the User.properties from there
    */

    // the return type can be suggested rather than written deliberately
    // you can just go ahead and put the None / Some parts in first before picking a return type,
    // and then you can use CTRL + SPACE at the -> part and it will suggest Option<f32> for you
    fn divide_by_number(dividend: f32, divisor: f32) -> Option<f32> {
        if divisor == 0.0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }

    println!("{:?}", divide_by_number(50.0, 5.0));
    println!("{:?}", divide_by_number(50.0, 0.0));

    let result = divide_by_number(100.0, 20.0);

    match result {
        Some(value) => println!("The result was {}", value),
        None => println!("Wasn't able to divide by zero"),
    }

    //the if let statement allows for only checking some possible variants instead of all of them
    let result2 = divide_by_number(1000.0, 33.0);

    if let Some(value) = result2 {
        println!("the result was {value}");
    }

    /*
        Enums Challenge:

        You are developing a program to charge parking prices for different types of vehicles in a parking lot

        Each vehicle can be a Car, Truck, or Motorcycle

        Cars can be different types such as SUV, Sedan, Coupe

        Trucks have cargo capacity in tons


        Your task is to create enums to represent the different types of vehicles and car types

        use pattern matching to calculate parking rates

        and then use these in the main program to print the parking rates for any vehicles of your choice

        vehicle type        subtype     parking rate
        Car                 SUV         20
        Car                 Sedan       15
        Car                 Coupe       10
        Truck               > 10 tons   25
        Truck               <= 10 tons  20
        Motorcycle                      10

    */

    //1. define an enum called CarType with variants SUV, Sedan, and Coupe

    #[derive(Debug)]
    enum CarType {
        SUV,
        Sedan,
        Coupe,
    }

    #[derive(Debug)]
    enum CargoCapacity {
        Over10Tons,
        AtOrUnder10Tons,
    }
    //2. define an enum named vehicle with variants Car, Truck, and Motorcycle
    #[derive(Debug)]
    enum Vehicle {
        Car(CarType),
        Truck(CargoCapacity),
        Motorcycle,
    }

    //3. Implement a method on the Vehicle enum named parking_rate that calculates and returns the parking rate for each vehicle
    impl Vehicle {
        fn parking_rate(&self) -> i8 {
            let parking_rate;

            match self {
                Vehicle::Car(cartype) => match cartype {
                    CarType::SUV => parking_rate = 20,
                    CarType::Sedan => parking_rate = 15,
                    CarType::Coupe => parking_rate = 10,
                },
                Vehicle::Truck(cargocapacity) => match cargocapacity {
                    CargoCapacity::Over10Tons => parking_rate = 25,
                    CargoCapacity::AtOrUnder10Tons => parking_rate = 20,
                },
                Vehicle::Motorcycle => parking_rate = 10,
            }

            parking_rate
        }
        
    }

    //4. In the main function, create a few vehicles

    let mut vehicles = vec![];

    vehicles.push(Vehicle::Car(CarType::SUV));
    vehicles.push(Vehicle::Motorcycle);
    vehicles.push(Vehicle::Truck(CargoCapacity::Over10Tons));
    vehicles.push(Vehicle::Truck(CargoCapacity::AtOrUnder10Tons));

    //5. Print the parking rate of the vehicle using the parking_rate method
    for vehicle in vehicles {
        println!("{:?}, the rate is {}", vehicle, vehicle.parking_rate());
    }



    /*
    
        The above challenge piece was my way of doing it before having seen the instructor's solution example


        After seeing the solution video, these are details I found interesting to note:

            For the truck variant, instead of having a further enum of two possible cargo capacities, the instructor used u32 so it could have any number given, and then I suppose they are about to use conditional branches to solve whether it's over or under 10,000

            the instructor's match statement was a bit more comact, looking like this:

            fn get_parking_rate(vehicle: Vehicle) -> i8 {
                match vehicle {
                    Vehicle::Motorcycle=>10,
                    ...
                }
            }

            the instructor also didn't use an impl block to make it a method that has &self, he made a function that takes Vehicle as an argument

     */

}
