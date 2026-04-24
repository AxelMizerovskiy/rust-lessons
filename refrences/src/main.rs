fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocekt fuel is {rocket_fuel} and length is {length}");


    // slices
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    let last_word = &message[15..15+5];
    println!("last_word is {last_word}");


    let s = "     hello world       ";
    let trimmed = trim_spaces(s);

    println!("trimmed string is {trimmed}");

}

fn process_fuel(propellant: &mut String) -> usize{
    println!("processing propellant {propellant} ...");
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}



// Borrowing Ownership
// lets us access data without taking ownership of it
// can do this by creatign ar refrence to it with
// borrow operator : &
// variables on the heap usually passed by refrence
//
// have to make refrences mutable if you want to change them
//
// Restriction:
// if a &mut exists cant make another & in the scope to same var
// prevents data races
//
// cant return borrrowed value since they would be out of scope
// ^ dangling refrence (refrence out of scope, stale pointers)
//
//
// Slice:
// borrow from an array
// refrence to a contigous section of a collection
// Common : string slice data type of &str 
// string literal are slices
//
// String Slices:
// length is in bytes
// some char could be more than 1 byte since we use UTF-8 char
// range indice must be a valid char boundry 
// basic letters are 1 byte, special char or emojis could be more 


fn trim_spaces(dirty: &str) -> &str{
    let mut start = 0;
    let mut end = dirty.len();

    for letter in dirty.chars(){
        if letter.is_whitespace(){
            start += letter.len_utf8();
        } else {break;}
    }

    for letter in dirty.chars().rev() {
        if letter.is_whitespace(){
            end -= letter.len_utf8();
        } else {break;}
    }

    &dirty[start..end]
}
