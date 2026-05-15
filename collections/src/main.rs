use std::{env::{self, args}, fs}; // challange

use std::collections::HashMap;
fn main() {
    // Vectors :)
    // Vec<T> Data Type
    // collection of elements with same type in order
    // vectors are dynamically alllocated on heap
    // have to handle ownership and borrowing
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard")); // Alan Shepard
    astronauts.push(String::from("Grissom")); // Gus Grissom
    astronauts.push(String::from("Glenn")); // John Glenn
    println!("astronauts are {:?}", astronauts);

    let last = astronauts.pop();
    println!("last is {:?}", last);

    // let third = &astronauts[2]; // borrowing
    let third = astronauts.get(2); // returns enum
    println!("Third is {third:?}");

    //let countdown = vec![5, 4, 3, 2, 1];

    // Hash Maps Data Type
    // HashMap<K, V> Data Type
    // stores data in key value paits
    // look up with keys 
    // key -> value mapping is one way
    // uses a hash function to determine how to store data
    // key and data can be diffrent data types 
    // all key have to be same type
    // all values have same type
    // unique keys and only one value per key
    let mut missions_flown = HashMap::new(); // missions flown as of 1st Jan 2021
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 0);
    println!("missions_flown are {missions_flown:?}");

    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {barron_missions:?}");

    // Changing values in hash map
    // 1. Overwrite an existing key val pair
    // 2. Insert new entry if ket doesnt exist
    // 3. modify a value based on its existing value

    // 1.
    missions_flown.insert("Barron", 1);
    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {barron_missions:?}");

    // 2.
    missions_flown.entry("Barron").or_insert(2); // entry already existed
    let barron_missions = missions_flown.get("Barron"); // so it doesnt do anything
    println!("barron_missions is {barron_missions:?}");

    missions_flown.entry("Stone").or_insert(2);
    let stone_missions = missions_flown.get("Stone"); 
    println!("stone_missions is {stone_missions:?}");

    // 3.
    let kayla = missions_flown.entry("Barron").or_insert(0);
    *kayla += 1;
    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {barron_missions:?}");


    // Final Challange
    // read a given text file
    //  - as a command line argument
    //  - error handeling as needed
    // parse text into individual words
    //  - split_whitespace method 
    //  - ignore cpitalization
    // counts number of times each word occures
    // prints most common word or words and how times they appeared
    if args().len() <= 1 {
        println!("Needs text path as an argument");
        return;
    } 
    let mut word_list = HashMap::new();  
    let path = match env::args().nth(1){
        Some(s) => s,
        None => {
            println!("Could not get path");
            std::process::exit(1);
        }
    };
    let contents = match fs::read_to_string(path){
        Ok(s) => s,
        Err(e) => {
            println!("Could not open and read file: {e}");
            std::process::exit(2);
        }
    };
    for word in contents.split_whitespace(){
        *word_list.entry(word.to_lowercase()).or_insert(0) += 1;
        // gets entry or if it doesnt exist add 0 then does + 1
    }
    // use if let matching to get highest counts
    if let Some(&max_count) = word_list.values().max(){
        // get all the words with max counts
        
        let winners: Vec<&String> = word_list
            .iter()
            .filter(|&(_,count)| *count == max_count)
            .map(|(word, _)| word)
            .collect();
        println!("Highest count for words is {max_count}");
        println!("The winning words are {winners:?}");
    }    


}
