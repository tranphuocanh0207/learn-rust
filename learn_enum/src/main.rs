#[derive(Debug)]
enum Language {
    Solidity,
    Move,
    Rust
}

#[derive(Debug)]
enum Coin {
    Bitcoin,
    Solana,
    Ethereum,
    Aptos,
    Sui(Language),
}

fn main() {
    let sui = Coin::Sui(Language::Move);
    println!("{:?}", sui);

    let six = plus_one(Some(5));
    println!("{:?}", six);
    let none = None;
    let plus_none = plus_one(none);
    println!("{:?}", plus_none);
}

fn plus_one(num : Option<i32>) -> Option<i32> {
    match num {
        Some(a) => Some(a + 1),
        _ => None
    }
}