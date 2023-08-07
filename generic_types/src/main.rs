use std::cmp::PartialOrd;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let largest1 = largest(&arr);
    println!("The number largest in the array is: {}", largest1);

    let arr2 = vec!['a', 'b', 'y', 'c', 'd'];
    let largest2 = largest(&arr2);
    println!("The char largest2 in the array2 is: {}", largest2);

    let point_1 = Point { x: 1, y: 3 };
    println!("The point 1 is : {:?}", point_1);
    let point_2 = Point { x: 2, y: 5.1 };
    println!("The point 2 is : {:?}", point_2);

    println!("The point x is : {}", point_1.x());

    let point_3 = Point { x: 1.2, y : 3.1};

    println!("Distance from origin is {}", point_3.distance_from_origin());

    let point_4: Point<i32, f64> = point_1.mixup(point_2);

    println!("The point 4 is : {:?}", point_4);
}

fn largest<T: PartialOrd>(arr: &[T]) -> &T {
    let mut largest = &arr[0];

    for i in arr {
        if i > largest {
            largest = i;
        }
    }
    largest
}
