use std::mem;
use std::ops::Add;
#[derive(Debug)]
struct Rectangle<T, U> { // add this for Generic type variables
    width: T, // T is now generic type 
    height: U
}

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl<T, U> Rectangle<T, U> { // tell the compiler that you implemnt with 2 generic
    fn get_width(&self) -> &T { // return refrence cause dont know if data type
                                // is heap or stack
        &self.width
    }
}

impl Rectangle<u8, u8> { // by not including generic types after implement we tell it 
                      // its only for specific concrete type of rectangle 
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
            
        
    }
}

fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> 
{
    Box::new(*a + *b)
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T { // have to define how to compare
                                                 // by setting limits to what data 
                                                 // type T can be
                                                 // such as all numbers 
    if a > b {
        a
    } else {
        b
    }
}


fn main() {
    let rect = Rectangle {
        width: 1u8, // can specify type 
        height: 3u8 // still needs to be the same type for T

    };
    println!("rect is {:?}", rect);


    // Generic Data Types
    // if you want a struct to be able to use other data types 
    // abstact standin for concrete data types or other properties
    // can be used with structs, functions, methods and more
    // Defined with <T>

    // Runtime Performance
    // Generics are a zero-cost abstraction
    // make programming easier without reducing runtime performance


    // Monomorphization
    // compiler replaces generic placeholders with comcrete data types 
    // the compiler copies defentitions to fit the data types

    println!("width is {}", rect.get_width());

    // can define methods that only apply to specific type of rectangle 
    
    println!("perimeter is {}", rect.get_perimeter());

    // Can also do generic functions and not just methods
    
    println!("biggest is {}", get_biggest(1, 2));

    // Box<T> Data Type
    // store data on the heap instead of the stack 
    // Box contains to a stack pointer to the heap 
    // where large enough space of memeory has been allocated 
    // to hold whatver generic type T is 
    //
    // Considered to be smart pointers because they provide
    // functionality beyond refrences like:
    // the box has ownership of data it points to
    // when box goes out of scope it deallocates the heap memeory
    
    
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("vehicle size on the stack: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    // allocates necessary ammount of mem on heap for a Shuttle
    // places existing instance into it 
    // not copy so vehicle looses ownership
    // boxed_vehicle becoems new owner

    println!("boxed_vehicle size on the stack: {} bytes", mem::size_of_val(&boxed_vehicle));

    println!("boxed_vehicle size on the heap: {} bytes", mem::size_of_val(&*boxed_vehicle));
    // Derefrence operator * 
    // when applied to a pointer it denotes the pointed to location
    

    // If want to move data back from heap to stack 
    // derefrence it

    let unboxed_vehicle: Shuttle = *boxed_vehicle; // passes ownership
    println!("unboxed_vehicle size on the stack: {} bytes", mem::size_of_val(&unboxed_vehicle));

    // use cases for Box<T>
    // store a data size whose size cannot be known at compile time
    // recursive types struct that has a struct, of the same type, 
    // as one of its fields
    // dont know how many layers at compile time
    //
    // Another use case is transfering ownership of data rather than copy it on the stack 
    // avoid copying large ammounts of stack data 

    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    println!("Tests passed!");

}
