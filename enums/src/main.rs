#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64) // sides a, b, c
} // like categories i guess 

 // Challange
enum Location{
    Known(f64, f64), // latitude, longitude
    Anonymous,
    Unknown
}

impl Location {
    fn display(&self){
        match *self {
            Location::Known(lat, long) => println!("Location is at {lat} and {long}"),
            Location::Unknown => println!("Location is unknown :( "),
            Location::Anonymous => println!("Location is Anonymous (scary)")
        }
    }
}
// challange over

impl Shape {
    fn get_perimeter(&self) -> f64{
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 *  h),
            Shape::Triangle(a, b, c) => a + b + c 
        }
    }
}

fn main() {
    // Enums
    // a data type with multiple possible variants 
    // enumurates a finite number of objects or items
    
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    // could enum commands sent to a draw func
    // also used in match operator



    // Match Operator:
    // compares a value to a series of paterns to determine
    // which code to exe
    // like a switch statement or a coin sorting machine


    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {r}"),
        Shape::Rectangle(w, h) => println!("{w} x {h} Rectangle"),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {a}, {b}, {c}")
    }
    
    // match doesnt only match clothes

    let my_number = 5u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        _ => { // wild card to cover all the other cases 
            println!("{my_number} did not match");
            "something else"
        }
    };

    println!("result is {result}");

    // Methods on enums
    // like defining methods for structs 
    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {perimeter}");

    // How to represent nothing?
    // other lang have null to indicate no value, not safe when used somehwere else
    // rust doesnt have null, instead has Option<T> Enum
    

    // Option<T> Enum
    // generic enum that could be 1 of two variants
    // Some(T) or None
    // included in prelude
    // enum Option<T> {
    //  Some(T),
    //  None
    // }

    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5); // get returns an Option enum, holding value 
                                   // to refrence
    //let number = number.unwrap_or(&0) + 1; // could use unwrap to get value but crash when None
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };

    println!("number is {:?}", number);

    // match Expressions and enums
    // often used together 


    // If-let syntax
    let num = Some(13);

    //match number {
    //    Some(13) => println!("thirteen"),
    //    _ => () // dont care about anything else 
    //} 
    //easier way to do this ^ is:
    if let Some(13) = num {
        println!("thirteen");
    }

    // Location Challange
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
