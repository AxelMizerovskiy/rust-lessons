use std::any;
use std::fmt; // format modules for display
// Traits
// abstract way to define capabilities or func of specific data types
// a collection of methods 
// implementing traits = implementing those methods 
// generic types use traits to specify the capabilites of unkown data types
// similar to interfaces in java or C++
// standard traits come with rust or can define your own
#[derive(PartialEq, PartialOrd)] // equal if only all fields equal
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

trait Description { // defining new trait
    fn describe(&self) -> String{ // default defenition 
        String::from("an object flying through space!")
    
    } 
    // can have mulitple methods
    // struct implementing Description will
    // inherit this method
}

impl Description for Satellite {} // now uses defualt 
                                  
impl Description for SpaceStation {
    fn describe(&self) -> String {
         format!("the {} flying {} miles high with {} crew members on board!", self.name, self.altitude, self.crew_size)
    }
}

fn print_type<T: fmt::Debug>(item: T) { // restirtcs to types that can be Displayed
    println!("{:?} is {}", item, any::type_name::<T>()); // item needs to have Display
}

// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U) 
    where   T: fmt::Display + PartialEq + From<U>, 
            U: fmt::Display + PartialEq + Copy
    { // have aware bounds to not crowd fn def 
    // seperate traits with + 
    if a == T::from(b){ // consumes b into a new T type var, so need U to have Copy
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn get_displayable(choice: bool) -> impl fmt::Display { // return any that implements display
    if choice {
        13
    } else { // data type determined at compile time so needs sto be same
        //"thirteen" 
        12
    }
}

// Trait Challange Type
impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a Satelite named {} flyign at {} miles per second", self.name, self.velocity)
    }
}



fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());

    println!("hubble == gps is {}", hubble == gps); // cant compare them without
                                                    // deriving an eqaulity trait
    // Derivable Traits
    // provide default implementations for several common traits
    // compiler generates default code 
    // Eq   PartialEq   Ord     PartialOrd  Clone   Copy    Hash    Default  Debug
    
    println!("hubble > gps is {}", hubble > gps); 
    // compares the values in order accoarding to defenition
    // so compared string values 
    // if you wanna compare diffrently just implement the traits yourself
    
    

    // Tait Bounds
    // require a generic type to implement specific traits 
    // guarantees the generic type will have necessary behaviors
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]); // array with 13 doesnt impl display so cant print
                            // does have debug tho so can use that to print
                            
    // Multiple Trait Bounds on Generic
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);

    
    // Return Types with Implemented Traits
    // out of a fn
    
    println!("output is {}", get_displayable(true));
    // if you need to return data but dont know type 
    // dynamic dispatch ( not in this course)
    


    // Challange
    // make display trait for Satellite
    println!("hubble is {}", hubble);
}

