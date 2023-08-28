use crossbeam::channel::unbounded;
use std::thread;
use std::time::Duration;

enum Message {
    PrintMsg(String),
    Sum(i32, i32),
    Quit,
}

enum MainMessage {
    Sum(i32),
    Quit,
}

fn main() {
    let (s, r) = unbounded();
    let (main_s, main_r) = unbounded();

    let a = thread::spawn(move || {
        loop {
            match r.recv() {
                Ok(msg) => match msg {
                    Message::PrintMsg(value) => println!("{}", value),
                    Message::Sum(a,b) => {
                        println!("Send arguments to main message!");
                        main_s.send(MainMessage::Sum(a + b));
                    },
                    Message::Quit => {
                        println!("Move to main message!");
                        main_s.send(MainMessage::Quit);
                        break;

                    }
                },
                Err(e) =>{ 
                    println!("Error: {:?}", e);
                    break;
                },
                
            }
        }
    });

    s.send(Message::PrintMsg("I'am Alan Tran".to_string()));
    s.send(Message::Sum(1,2));
    s.send(Message::Quit);

    while let Ok(msg) = main_r.recv() {
        match msg {
            MainMessage::Sum(a) => println!("Sum from main: {:?}", a),
            MainMessage::Quit =>{ 
                println!("Quit from main!");
                break;
            }
        }
    }

    a.join();
}
