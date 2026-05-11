struct Shuttle<'a> { // dont know how to treat its lifetime after struct
    name: &'a str
} // changes the data type so impl needs to be same

impl<'a, 'b> Shuttle<'a> {
fn send_transmission(&'a self, msg:&'b str) -> &'b str { // didnt need lifetime here cause rule #3
    println!("Transmitting message: {}", msg);
    msg
}
}

fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
if x.len() > y.len(){
    x
} else {
    x 
}
}
// instead of defining the generic type we define generic lifetime
// tells compiler how the lifetimes of the variables relate to each other
// tells rust the lifetime of the return would be as long as lifetime x or y
// whichever is smaller
// used for borrow checker 
// all same lifetime 


fn main() {
    // rust compiler is crazy goated
    // amazing notes for your errors
    
    // How?
    // Borrow Checker
    // compares the scopes of varaibles to determine if borrows valid
    // 'a to label life times ie 'letter
    // rust considers the scope of variables used but also 
    // the scope or lifetime of refrenced values
    
    
    // SOmetimes have to specifically tell how long lifetime will be
    
    
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
    // need borrowed lifetime refrence
    // Lifetime annotation
    // explicity feines generic lifetime for parameters 
    // begins with '
    // convention is single lwoer case name
    

    // Lifetime Elision Rules
    // compiler infers lifetimes so that you dont have to define one each time 
    // set of rules for the compiler to analyze refrence lifetimes
    // describe situations that do not require explicit lifetime annotations
    // if ambigious, explicit annotation still required
    //
    // Lifetimes ONLY for refrences 
    //
    // Rule #1:
    // each input parameter is assigned its own lifetime
    //
    // Rule #2:
    // If there is one input lifetime, assign that to all output lifetimes
    //
    // Rule #3:
    // If there is a &self or &mut self input parameter its lifetime 
    // will be assigned to all output lifetimes
    //
    // If after rules comipler still doesnt know output lifetime it asks for explicit
    // no harm in including explicit anyways 
    // can add more rules

    let vehicle = Shuttle {
        name: "Endeavor"
    };
    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);

    // Static Lifetime
    // 'static
    // reserved 
    // indicates that data is availebel for the entire duration of the program
    // example: string literals
    // can be coerced to be a more restictive lifetime if refrenced passed as output
    // parameter
    // ensures the data type will only contain static elements - as a Trait Bound
    // T: Display + 'static
    // all T data has static

}
