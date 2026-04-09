fn main() {
    // functions
    say_hello();
    say_hello();
    say_a_number(21);
    let x = 1; // this is not initialized as u8 since thats what we use it as
    let y = 2;
    say_the_sum(x,y);

    let result = square(13);
    println!("result is {:?}", result); //debug formatting handles touples
    

    let cel_temp = 23.0;
    let fah_temp = cel_to_fah(cel_temp);

    assert_eq!(fah_temp, 73.4);
    println!("Test Passed! {fah_temp}");
}

fn say_hello(){ // doesnt matter where function delcared
    println!("Hello World!");
    say_a_number(13); // argument
}

fn say_a_number(number : i32){ // parameter
    println!("Number is {number}");
}

fn say_the_sum(a: u8, b: u8){
    let sum = a + b;
    println!("sum is {sum}");
}


// statment performs an action without return. ends with semicolon 
// expression do not end with semicolon, usually part of statement, have return
//


fn square(x: i32) -> (i32, i32){ // the arrow is how you show it returns something
    println!("squaring {x}");
    return (x , x * x);
} // in rust when you leave last line as an expresion that is passed down as return
  //ie x * x
  //
  // can also just doo return val;
  //
  // for returning a touple just change i32 to (i32, i32)
  //
  //
  //
  // when nothing to return it returns Unit Data Type, represented with (), 
  // can do -> () for return



fn cel_to_fah(cel : f64) -> f64{
    (1.8 * cel) + 32.0
}
