use std::collections::HashMap;

fn vector() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("vector is {:?}", v);

    let v2 = vec![4, 5, 6];
    println!("vector v2 is {:?}", v2);

    let first_element = &v[0];
    println!("first element is {:?}", first_element);
    let third_element = v.get(2);
    match third_element {
        Some(third) => println!("third element is {:?}", third),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{i}");
    }

    let mut v3 = vec![2, 3, 4];
    for i in &mut v3 {
        *i += 10;
    }
    println!("{:?}", v3);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.0),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

    println!("row is {:?}", row);
}

fn string() {
    let mut s = String::from("Hello world!");
    println!("String is {}", s);
    s.push_str("I'm Alan Tran");
    println!("String after push str is {}", s);
    s.push('!');
    println!("String after push letter is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;

    let x1 = String::from("tic");
    let x2 = String::from("tac");
    let x3 = String::from("toe");
    let x = format!("{x1}-{x2}-{x3}");
    println!("X String is {}", x);

    for c in x.chars() {
        println!("{}", c);
    }

    for c in x.bytes() {
        println!("{}", c);
    }
}

fn hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    println!("{:?}", scores);

    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("{}", score);

    for (key, value) in &scores {
        println!("key is {} and value is {}", key, value);
    }

    scores.insert(String::from("Yellow"), 50);
    println!("Hashmap after insert value is {:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { 
        let count = map.entry(word).or_insert(0);
        *count +=  1;
    }
    println!("Map is {:?}", map);
}

fn main() {
    vector();
    string();
    hashmap();
}
