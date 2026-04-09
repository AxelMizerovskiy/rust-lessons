fn main() {
    let mut x: u8 = 10; // automatically assigns type i32 default
    println!("x is {}", x); // print macro cause of the !
    x += 1;
    println!("x is {}", x);
    //rust only has f32 and f64 for floating point. 32 -> 64 more precision. 6-9 to 13-15 digits

    let y: f32 = 10.12345678901234567890; //now a f32, f64 by deafult
    println!("y is {}", y);
    let a = 10;
    let b = 3.0;
    // Casting is when you do as f64 or i32 on a variable to have it treated as another type
    let c = a as f64 / b; // + - * / all included, also integer division here applies
    let d = a as f64 % b; // you cannot do float / int since its ambigous 
    // when casting float to integer it would be truncated instead of roundinded 
    // 300 as u8 = 44
    // -300 as u32 -> 4294966996
    // * / % before + -. Have to use parenthesis otherwise
    println!("value of c is {}", c);
    println!("calue of d is {}", d);

    // print formating
    println!("--- Print Formatting ---");


    println!("c is {:.3}", c); // how many digits after . to show
    println!("c is {:8.3}", c); // how many  total to have for the variable
    println!("c is {:08.3}", c); // change what char to pad with
    println!("c is {:08.3} \na is {}", c, a); // adding more variables
    // print and print ln both work 
    // position parameters
    println!("c is {0:08.3} \na is {1}\nonce again c is {0}", c, a);
    // this also works now from 1.58 rust
    println!("d is {d}");

    // can also use binary to represent numbers with underscores
    let mut bin = 0b1111_0101; // underscores are optional val stored at i32
    let bin2 = 0b1111_0101u8; // can store val in an unsigned 8 bit
    println!("bin is {bin} and bin2 is {:08b}", bin2); // can show binary with this
    // bitwise operators 
    println!("--- Bitwise Operators ---"); 
    //not
    bin = !bin;
    println!("bin is now : {bin}");

    //and 
    // to clear bit at position 2
    println!("bin is now : {:08b}", bin & 0b1111_1011);

    // to check value of bit 1
    println!("bit 1 is : {:08b}", bin & 0b0000_0010);
    
    // or
    // can set a specific bit to one, 0 for everything but the set value
    bin = bin | 0b0100_0000; // changes bit 6 to 1
    println!("bin is now : {bin:08b}"); // works fine just i think i increased it to use more bits
    
    // xor 
    // can be used to check where patterns differ 
    bin = bin ^ 0b0101_0101;
    println!("bin val is {bin:08b}");

    //bitwise shifts
    // shifts in zeros fill in spaces
    
    println!("--- Bitwise Shifts ---");

    bin = bin << 4; // shifts to the left by 4
    println!("bin is now : {bin:08b}");

    bin = bin >> 2; // shifts right by 2 bits
    println!("bin is now : {bin:08b}");

    // boolean values;
    println!("--- Boolean Values ---");
    
    let o = true;
    let p = false; // outputs text values when printed
    
    println!("o is {} and p is {}", o , p);            
    println!("Not o is {}", !o);
    println!("o AND p is {}", o & p);
    println!("o OR p is {}", o | p);
    println!("o XOR p {} ", o ^ p);
    
    let q = (o ^ p) | (o & p); // can use || and && for short circuiting
                        // ie only look at left side to figure out if true or not
                        // can use panic!() to immediatly terminate with error

    println!("q is {}", q); 


    // comparison operators
    println!("\n--- Comparison Operators ---");
    let r = 1;  // can also compare booleans like that but cant mix data types 
    let s = 2;

    println!("r is {r} and s is {s}");
    println!("r is EQUAL to s is {}", r == s);
    println!("r is NOT EQUAL to s is {}", r != s);
    println!("r is GREATER THAN s is {}", r > s);
    println!("r is GREATER THAN OR EQUAL to s is {}", r >= s); 
    // less than works the same so im not gonna write it 
    
    // char data type
    println!("--- Char Data Type ---");
    // unicode scalarvalue
    // stored using 4 bytes instead of 1 like in C++ so can do more
    
    let letter = 'a'; //these are characters now cause of the single quotes
    let number = '1'; 
    // can do emotes\
    
    let finger = '\u{261D}';

    println!("{}\n{}\n{}", letter, number, finger);


    // mean challange
    let t = 13;
    let u = 2.3; //already f64 so dont need to cast it 
    let v: f32 = 120.0;

    //code
    let average = (t as f64 + u + v as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");


    //compount data types
    println!("--- Compound Data Types ---");
    // array and touple
    // array can hold same data type elements in order
    // contigous memory location, fixed length

    let mut letters = ['a', 'b', 'c' ];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers : [i32; 5]; // makes array of 5 signed integers
    numbers = [0; 5]; // this is a repeat expresion, makes array of 5 0s
    let index: usize = numbers.len() - 1 as usize; // index has to be usize and usize len is 
                                      // calculated at compile time by compiler
                                      // index returns length so 5 and not 4
    println!("Last number is {}", numbers[index]);


    //multi-dimension arrays
    println!("--- Multi-Dimensional Arrays ---");
    
    let parking_lot = [[1,2,3],
                       [4,5,6]];// this has to have the same data types
                                // so one cant be 3 int array and second be 4 int
                                // array since those are different types
    let spot = parking_lot[1][2];

    println!("spot is {spot}");

    let garage : [[[i32;100];20];5] = [[[0; 100]; 20]; 5]; // 5 x 20 x 100
    // or let garage = [[[0; 100]; 20]; 5];


    // tuple data type
    println!("--- Tuple Data Type ---");
    // can have mixed data types
    // ordered but that ususally doenst matter
    // stored fixed length contigous section mem
    // data types still need to be know at compile time
    

    let mut stuff : (u8, f32, char) = (10, 3.14, 'x'); //dont have to define types but can
    stuff.0 += 3; 
    let first_item = stuff.0; // so kinda like a struct instead of array
    println!("first item is {}", first_item); 
    
    //can also do destructering which assaigns each value in stuff to a variable
    let (w, x, y) = stuff;
    println!("x is {x}");





}
