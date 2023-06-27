use std::io;

fn main() {
    let array = [1,2,3,4,5,6];

    println!("Enter an array index:");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index : usize = index.trim().parse().expect("Index entered was not a number!");

    let element = array[index];
    
    println!("The value of the element at index {index} is {element}");

    another_function();

    another_function_with_x(1);

    print_labeled_measurement(5 ,'h');

    let x = plus_one(5);

    println!("The value of x is {x}")
}

fn another_function() {
    println!("Another function called!");
}

fn another_function_with_x(x : u32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, uint_label: char) {
    println!("The measurement is {value}{uint_label}");
}

fn plus_one(x : i32) -> i32 {
    x + 1
}