fn main() {
   let x = 3;
   let y = 5;

    if x - 1 != 3 { // cant use integers is bool for data
        println!("x is NOT 3");
    }

    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }


    // in rust if creates an expresion, so you can set things to if 
    let max_h_odd = true;
    let h = if max_h_odd{1} else {2};

    println!("h is {h}");


    // rust has loop while and for loops 
    // loop is basically just while true
    // you can pass out values out of a loop after it breaks
    let mut count = 0;
    let result = loop {
        count += 1;
        println!("count is {count}");
        if count == 10 {break count * 10;}
    };
    println!("After the loop, result is {result}!");
    // rust has break


    // also has while
    // evaluates condition before code
    let mut counts = 0;
    while counts < 10{
        counts += 1;
        println!("counts is {counts}");
    }

    // loop and wqhile true seem same but actually NOT
    // loop break can return a value while can't 
    
    count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len(){
        println!("letter is {}", letters[count]);
        count += 1;
    }

    // for loops good for indexing arrays

    let message = ['h', 'e', 'l', 'l', 'o'];
                           //can do a .enumerate() here which makes it a touple with an index
    for item in message { // this converts message to iterator and uses next 
        println!("item is {item}");
    }

    for (index, &item) in message.iter().enumerate(){
        println!("item is {}, and its index is {}", item, index);
        if item == 'e'{break;}
    }

    for number in 0..5{ // 0 - 4 
        println!("number is {number}");
    }

    // choosing loops 
    // loop = for reapeating code forever, return value on exit
    // while = repeat code as long as condition is true
    // for = iterate over each item in collection
    
    
    let mut matrix = [[1, 2, 3],[4, 5, 6], [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{num} \t");
        }
        println!();
    }

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    
    let mut max: i32 = numbers[0]; 
    let mut min: i32 = numbers[0];
    let mut mean: f64 = 0.0;

    for num in numbers{
        if num > max {max = num;}
        if num < min {min = num;}
        mean += num as f64;
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");

}
