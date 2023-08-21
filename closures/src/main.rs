use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let give_away1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, give_away1
    );

    let user_pref2 = None;
    let give_away2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, give_away2
    );

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrow = || println!("From closure: {:?}", list);
    println!("Before defining closure: {:?}", list);
    only_borrow();
    println!("After defining closure: {:?}", list);


    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("After defining closure: {:?}", list2);


    let mut listRec = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 7, height: 2},
        Rectangle { width: 8, height: 3 }
    ];
    let mut count = 0;
    listRec.sort_by_key(|r| {
        count += 1;
        r.width
    });
    println!("{count}");
    println!("{:?}",listRec)

}
