use std::io;
use rand::prelude::*; //wildcard, everything in rand prelude

fn main() {
    // Rust Standard Library
    // has core data types
    // functions
    // macros
    // + other things

    // to use modules and libraries
    // use use 
    // to bring them into scope
    // part of std module != part of rust lang 
    // use std::name

    // prelude - list of things auto imported into every rust prog
    // not entire rust std, just most common
    // outside prelude = import with use
    
    // Standard input
    // read command line input from the user
    // part of std::io module
    
    let mut buffer = String::new();

    // since string type, its on the heap so dynamically resized
    
    println!("Enter a message: ");
    //io::stdin().read_line(&mut buffer);
    println!("buffer is {buffer}");
    
    // parsing input String types
    // stdin Strings include a \n at the end

    // let number = buffer.trim().parse::<i32>().unwrap();
    // trim to remove whitespaces, parse to turn it into a diff datatype
    // turbofish op ::<i32> to tell which type
    // or just strictly define datatype for the variable
    // parse doesnt actually return i32 but an enum
    // could either contain i32 or Err for bad input
    // error handeling later
    // .unwrap() to extract the i32 from it 

    // println!("number + 1 is {}", number + 1);

    // Crates
    // collection of Rust source code files
    // binary crate vs library crates
    // - compile to produce executable programs
    // - all code in lessons is binary crates
    // vs
    // - collection of code for other programs to use
    // i.e. not exe 
    
    // crates.io official repo for crates
    // like a random num gen is not in standard lib (back then)
    // use rand crate from there
    // in otder to use it we have to add the package + version to cargo.toml
    // rand = "0.8.0"
    
    let number = random::<f64>();
    println!("number is {}", number);

    // cargo takes longer time to build since its retriving the dependecies
    // can import specific functions
    // can remove full path for that
    // gets ambigous if doing that

    let number = thread_rng().gen_range(1..11); // 1 - 10
    println!("number is {}", number);


    //Challange
    // guess higher or lower number

    let win_num = thread_rng().gen_range(1..101); 
    let mut guess_num = 0;

    loop {
        println!("Guess the number: ");
        io::stdin().read_line(&mut buffer);
        guess_num = buffer.trim().parse::<i32>().unwrap();
        // could have used expect here at the end instead of unwrap
        // to handle errors

        if guess_num < win_num {
            println!("Higher!");
        } else if guess_num > win_num{
            println!("Lower!");
        } else if guess_num == win_num{
            println!("You got it! It was : {win_num}");
            break;
        } else {
            println!("something went wrong");
        }
        buffer = String::new();
    }

}
