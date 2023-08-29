use std::rc::Rc;

#[derive(Debug)]
struct Car {
    number : String
}

#[derive(Debug)]
struct Door {
    vehicle : Rc<Car>,
}

fn main() {
    let car = Rc::new(Car { number : "71A - 123.45".to_owned() });
    let left_door = Door {
        vehicle : car.clone(),
    };
    let right_door = Door {
        vehicle : car.clone(),
    };
    println!("{:?}", car);
    drop(car);
    println!("{:?}", left_door);
    println!("{:?}", right_door);
}
