#[derive(Debug)] // something with traits
#[derive(Clone)] // for cloning String Types
struct Shuttle { // common to capitalize the name of struct
    name: String,
    crew_size: u8,
    propellant: f64
}
// defines what data looks like but with no real data present until intialized


// Tuple Struct
struct Color(u8, u8, u8); // RGB

struct Point(u8, u8, u8); // XYZ


impl Shuttle { // just seperate block to define methods in
    fn get_name(&self) -> &str { // returns slice
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64){
        self.propellant += gallons;
    }
    //by encapsulating the gallon cahnging we can add checkign logic
    //so that its never set to an invalid value 
    
    fn new(name : &str) -> Shuttle{ // constructor associated functions
        Shuttle {
            name : String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

fn get_y(p: Point) -> u8{
    p.1
}

// Challange 1:
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64{
        self.width * self.height
    }

    fn scale(&mut self, scaler: f64){
        self.width = self.width * scaler;
        self.height = self.height * scaler;
    }

    fn new(width: f64, height : f64) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    // Tuple Data Type
    // groups multiple items of mixed data types
    // ordered elements, access by order
    
    // Struct Data Type
    // is like a touple where it groups multiple items of mixed data types together
    // but its elements are named and you can access them through those names 

    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    println!("name is {}", vehicle.name);

    vehicle.name = String::from("Atlantis");

    println!("name is {:?}", vehicle);


    // struct is stored on the Stack, if string type then pointer on stack and 
    // real data on Heap ( like normal string type)
    //
    // String type is actualyl very similar to a struct
    
    // Struct only defined once

    //let vehicle2 = Shuttle {
        //name: String::from("Discovery"),
      //  ..vehicle // any fields not set have the same value as the vehicle
    //};
    
    // println!("name is {:?}", vehicle2);
    // any changes to vehicle wont change vehicle2 since they are seperate instances
    // cannot move String since it can only have one owner, doesnt affect values on 
    // the stack since they are copied,
    // one solution is to use a slice but lifetime complication
    // another soliution is cloning the values
    // shuttle doesnt natively have that trait so we have to derive it as well


    // Structs can also have methods:
    // (subroutines associated with a struct)
    // similar to functions but can manipulate the values inside the struct
    // declared with fn as well
    // can have input and return values
    //
    // one diffrence is that they are defined in struct 
    // and first paramter is always a refrence to the struct itself
    
    
    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {vehicle_name}");


    println!("propellant is {}", vehicle.propellant);

    vehicle.add_fuel(1000.0);

    println!("propellant is {}", vehicle.propellant);

    // associated functions
    // diffrence between this and methods is that associated functions
    // dont have a self perarmeter so you cant change datat inside the struct
    // can provide subroutines fot struct type in general
    // like a constructor for new instances 


    // Now you can do
    vehicle = Shuttle::new("Endeavour");
    let mut vehicle3 = Shuttle::new("Discovery");

    vehicle3.add_fuel(99.0);
    println!("{}", vehicle.name);

    // Tuple Structs
    // store a collection of mixed data without named fields
    // but acts as a unique data type 
    // good for when you want to give it a name but dont feel like naming every entry
    // color tuple 
    // (red, green, blue) < dont need to name each field since they are always stored
    // in that order 


    let red = Color(255,0,0);
    // access via relative index order

    println!("First value is {}", red.0);


    let coord = Point(4, 5, 6);
    let y = get_y(coord); // can only take in Point since they are defined as
                            // seperate data types 

    println!("y is {y}");



    // Challange 1:
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Test passed!");


}
