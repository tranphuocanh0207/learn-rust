trait Clicky {
    fn click(&self) -> String;
}

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) -> String {
        "Click keyboard".to_owned()
    }
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) -> String {
        "Click mouse".to_owned()
    }
}

fn main() {
    let a :Box<dyn Clicky>= Box::new(Keyboard);
    println!("{}", a.click());

    let b: Box<dyn Clicky> = Box::new(Mouse);
    println!("{}", b.click());

    let vec: Vec<Box<dyn Clicky>> = vec![a,b];

    for i in vec.iter() { 
        println!("{}", i.click());
    }
}
