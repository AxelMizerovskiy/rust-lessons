/* Scope
its a region of the program wgere a variable is valid
variable valid when it comes into Scope
remains valid until it goes out of scope

variables are bound to blocks of code
each block of code is enclosed by {} curly braces
    - functions, loops, conditionals and more 



*/
fn main() {
    let planet = "Earth"; // planet is only in main 
    if true{
        let planet = "Venus"; // planet is only in if, shadows
        println!("planet is {}", planet);
    } // you can just add random {} to make new blocks
    
    println!("planet is {planet}");

    {
        let mut planet = 4;
        planet += 1;
        println!("planet is {planet}");
    }

    //variabeles are not mutable by default
    //but we can create variables with the same name as a existing var
    //called shadowing
    // new variable masks the orig in the scope
    

    // Memory managment in rust
    // has stack and heap
    //
    // Stack values are stored in sequential order
    // data is LIFO
    // stack of boxes
    // push pop quickly, quick access
    // small size
    // all data known fixed size, no dynamically allocated
    //
    // Heap
    // giant warehouse
    // looks for enought space to hold data, allocates the spot
    // accessing is complicated since its not sequential
    // pointer is a data type that stores mem address,
    // small size so can be stored on stack
    //
    // can dynamically add or remove data
    // heap has a ton of space
    



    // new data type
    // String is a set of characters
    // stored on the heap
    // can have "yello" <- string literal, hard coded into exe
    // immutable, must know value since hardcoded
    //
    // string type
    // allocated on the heap 
    // mutable
    // dynamically chagned adn resized
    let mut message = String::from("Earth"); // path operator ;;
                                         // allows access from funct from string
                                         // from takes literal to intialize new string
                                         // stored in order on heap
    
    println!("message is {message}");
    // holds pointer to the first character, length and capacity
    message.push_str(" is home"); // grows string heap 
                                  // updates pointer since it had to
                                  // find new location
                                  // and length and capacity
    
    println!("message is {message}");

    // heap is bigger than stack but not infinite
    // have to clean up allocated mem blocks not needed 
    

    // Ownership
    // c++ uses manual mem alloc while python uses garbage collection
    // rust uses ownership 
    //
    // resources can only have one owner at a time
    
    // Example:
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury"); // assigned in scope 
        println!("inner_planet is {inner_planet}");
        inner_planet = String::from("Mars");
        outer_planet = inner_planet; // copies inner to outer
                                     // but only one owner allowed
                                     // so rust invalidates inner
                                     // transfers ownership
                                     // called a move
        // if we want it to copy data we do
        // outer_planet = inner_planet.clone();
        // makes two instances of inner on heap
        // if we change inner it wont affect outer since they are diff
    }
    println!("outer_planet is {}", outer_planet);


    let out_plan: i32;
    {
        let mut in_plan = 1;
        out_plan = in_plan; // since its int, clone isnt needed
                            // since its on the stack
                            // it just copies 
                            // since move and copy same efficency
                            // done for stack only data types
                            // int and floating point
                            // occures implicit, clone is explicit
        in_plan += 1;
        println!("in plan is {in_plan}");
    }
    println!("out_plan is {out_plan}");


    let rocket_fuel = 1;
    process_fuel(rocket_fuel); // passes in a copy
    println!("rocket_fuel is {rocket_fuel}");

}

fn process_fuel(mut propellant: i32){
    propellant += 1;
    println!("processing propellant {propellant}...");
}
// if this was a string type instead then the passesed in propellant
// would take ownership of rocket fuel unless explicitly cloned
// so pass in a clone of rocket_fuel
// can pur ownership back to rocket_fuel using return
// line 127 = let rocket_fuel = process_fuel(rocket_fuel);
// clone is deep copy
