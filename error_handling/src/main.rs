use std::fs; // file handler
use std::io; // error handler
use rand::prelude::*; // for rand gen


// this propogates the errors to the place where they were called
fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error>{
    let mut s1 = match fs::read_to_string(f1){
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    // This was of handeling errors is so common that rust has a shorthand for this:
    // can only use with methods returning result
    // elso enum error types have to match 
    // so this can only return io errors
    // rust by example has a section where you can learn how to reaturn 
    // multiple error types
    let s2 = fs::read_to_string(f2)?;
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}


fn main() {
    // Unrecoverable errors
    // errors are part of life
    // rust has several runtime error handlers
    // has two categories :
    // -recoverable : can continue running ex opening a non existing file
    // -unrecoverable : breaks program ex index beyong array bounds 
    // rust doesnt have exceptions to handle these 
    // has Result<T, E> enum type to handle recoverable
    // has a panic! macro for unrecoverable
    


    // panic! macro immediatly stops program and gives feeback on issue

    // panic!("Houston, we've had a problem.");
    
    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println!("T-minus {count}");
        // let x = 1 / count; // divide by 0, panics 
    }
    // use RUST_BACKTRACE=1 env varaible when running to see more details 
    
    // Recoverable errors 
    // errors that do not cause program to fail and can be corrected
    // ex. file not found 
    // could prompt user for a diffrent file or create missing file
    // for error handling result has 2 varibles
    // Ok = success, Err is = failed
    // included in prelude
    //
    // enum Result<T, E> {
    //  Ok(T),
    //  Err(E)
    // }

    let result = fs::read_to_string("the_ultimate_question.txt");
        // .expect("Nobody knows the ultimate question!"); 
        // // expect is basically unwrap but for error handlers
        // still
        // not 
        // as
        // good
        // as
        // match 
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of error")

        }
    };
    println!("contents are {:?}", contents);
    // read_to_string has potential for erro so it returns result enum
    
    // Could preform diffrent actions based on error type
    // enum srd::io::ErrorKind has those types

    let result = read_and_combine("planets.txt", "dward_planet.txt");
    match result {
        Ok(s) => println!("result is...\n{s}"),
        Err(e) => println!("There was an error: {e}")
    };

    // Challange
    // high or lower game with error handeling
    // if user enters invalid input print message to let user know to try again
    
    let mut buffer = String::new();

    let win_num = thread_rng().gen_range(1..101); 
    let mut guess_num = 0;

    loop {
        buffer = String::new();
        println!("Guess the number: ");
        io::stdin().read_line(&mut buffer);
        let input = buffer.trim().parse::<i32>();
        match input {
            Ok(s) => guess_num = s,
            Err(e) => {
                println!("Input a number. Error : {e}");
                continue;
            }
        };
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
    }


}

