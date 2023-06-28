fn main() {
    let s = String::from("hello");
    
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("{}", x);

    let some_string = gives_ownership();

    println!("{}", some_string);

    let some_string = takes_and_give_back(some_string);

    let (len , some_string) = calculate_length(some_string);

    println!("The length of {} is {}", some_string, len);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    String::from("Hello, world!")
}

fn takes_and_give_back(s: String) -> String {
    s
}

fn calculate_length(s : String) -> (i32, String) {
    (s.len() as i32, s)
}
