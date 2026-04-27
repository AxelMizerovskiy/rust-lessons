use std::{env::{self, args}, fs}; // filesystem
use std::io::prelude::*; // import traits
                    // has preulde



fn main() {
    if args().len() <= 2{
        println!("Program requires at least 2 arguments.");
        return;
    }

    // Command Line Arguments:
    // arguments passed to the program when its invoked
    // common uses :
    //  - file paths
    //  - config settings
    
    //  to read the config arguments you use std::env::args
    //  standard library env module
    //  returns an iterator over arguments passed to the program 
    //  first arg is traditionally the exe path (not gurenteted)
    for (index, argument) in env::args().enumerate(){
        println!("argument {index} is {argument}");
    }

    // to return a specific argument, use the nth method

    let arg2 = env::args().nth(2).unwrap(); 
    println!("arg2 is {arg2}");

    let contents = fs::read_to_string("planets.txt").unwrap();
    // returns enum so gotta unwrap
    println!("contents is {contents}");

    for line in contents.lines(){ // splits it into files
        println!("line is {line}");
    }

    // for images and video you can read the data into bytes
    
    let contents = fs::read("planets.txt").unwrap(); //vector of u8 vals 
    println!("contents is {:?}", contents); // since standard display cant display vectors 
    // read is a convient wrapper for lower level functions like file open
    // string literalls can hold paths
    // but can use path module for cross platform paths
    
    
    // Write to file 
    let mut speech = String::new();
    speech.push_str("We chose to go to the Moon in this decade\n");
    speech.push_str("and do the other things, \n");
    speech.push_str("not because they are easy, \n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech); // write to file
    // overwrites and replaces existing files
    // writes entire contents, so cant be a stream
    // convients again
    // create and write all methods 


   let mut file  = fs::OpenOptions::new().append(true).open("planets.txt").unwrap(); //new options for opening file
                                                                                    // append
                                                                                    // returns enum
                                                                                    // = unwrap
    file.write(b"\nPluto").unwrap(); // bytes becuase thats what write expects 


    // Challange 
    // program that checks if a specific person is on a list of names
    // print message with if name found or not
    // accept two command-line args
    //      - path to text file with list of names
    //      - name to search
    let file_name = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    let file_contents = fs::read_to_string(file_name).unwrap();

    for line in file_contents.lines(){
        if line == name{
            println!("Name Found!");
            return;
        }

    }
    println!("Name not found!");
     
    
}









