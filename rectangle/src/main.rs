#[derive(Debug)]
struct Rectangle {
    length: u64,
    width: u64
}

impl Rectangle {
    fn calculate_area(&self) -> u64 {
        self.length * self.width
    }

    fn calculate_perimeter(&self) -> u64 {
        (self.length + self.width) * 2
    }

    fn square(&self) -> Rectangle {
        Rectangle { length: self.length, width: self.length }
    }
}

impl Rectangle {
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rectangle_1 = Rectangle { length: 2, width: 1};
    println!("Length of rectangle is {:#?}",rectangle_1);
    let rectangle_area = rectangle_1.calculate_area();
    println!("Area of rectangle 1 is {}", rectangle_area);
    let rectangle_perimeter = rectangle_1.calculate_perimeter();
    println!("Perimeter of rectangle 1 is {}", rectangle_perimeter);
    let rectangle_2 = Rectangle {
        length: 3,
        ..rectangle_1
    };
    println!("Rectangle 2 is {:#?}",rectangle_2);
    let square = rectangle_2.square();
    println!("Square is {:#?}", square);
    println!("Square can hold rectangle 1: {}", square.can_hold(&rectangle_1));
    println!("Rectangle 2 can hold rectangle 1: {}", rectangle_2.can_hold(&rectangle_1));
}
