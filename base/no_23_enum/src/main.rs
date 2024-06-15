enum  Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Move to x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                println!("Write message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red: {}, green: {}, blue: {}", r, g, b);
            }
        }
    }
}

fn main() {
    let m = Message::ChangeColor(255, 0, 0);
    m.call();
}
