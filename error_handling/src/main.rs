use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;
use std::io::{self, Read};
use create_validation::{self, Guess};

fn main() -> Result<(), Box<dyn Error>> {

    // let username_result = read_username_from_file();
    // let username = match username_result {
    //     Ok(some_string) => some_string,
    //     Err(e) => {
    //         panic!("Error reading username from file: {:?}", e);
    //     } 
    // };
    let username = read_username_from_file()?;
    println!("username: {:?}", username);
    // let greeting_file_result = File::open("hello.txt");
    // println!("{:?}", greeting_file_result);
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Problem creating the file: {:?}",e);
    //             }
    //         },
    //         other_error => {
    //             panic!("Problem creating the file: {:?}",other_error);
    //         }
    //     }
    // };
    // println!("{:?}", greeting_file);
    let username2 = read_username_from_file_shorter()?;
    println!("username2: {:?}", username2);
    let guess = Guess::new(32);
    println!("guess: {:?}", guess);
    let guess_panic = Guess::new(101);
    println!("guesss panic: {:?}", guess_panic);
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}



// fn example_panic() {
//     panic!("cash and burn");
//     let v = vec![1, 2, 3, 4, 5];

//     v[10];
// }
