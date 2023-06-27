use std::io;

fn main() {
    println!("Enter your number:");

    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x : usize = x.trim().parse().expect("X Entered is not a number");

    let a =  if x < 10 {
        true
    } else {
       false
    };

    println!("The value of a is {a}");

    let counter = result_after_loop(10);
    
    println!("The value of result is {counter}");

    print_while();

    let (index, status) = index_of_value(6, &[1,2,3,4,5]);

    if status {
        println!("The index of value is {index}");
    } else {
        println!("Not exists");
    }
}

fn result_after_loop(x: usize) -> usize {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == x {
            break counter + 1;
        }
    };
    result
}

fn print_while() {
    let mut c = 4;
    while c != 0 {
        println!("Count down: {c}");
        c -= 1;
    }
}

fn index_of_value(number: usize, array: &[usize]) -> (usize, bool) {
    
    let mut i = 0;
    while i < array.len() {
        if array[i] == number {
            return (i, true);
        }
        i += 1;
    }
    return (0, false);

}
