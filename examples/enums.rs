fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // sel.
        }
    }

    let m = Message::Write(String::from("test"));
    m.call();
    let s = match m {
        Message::Write(s) => s,
        _ => String::from("")
    };

    let m2 = Message::Move { x: 42, y: 42 };
    let x = match m2 {
        Message::Move { x, y } => x,
        _ => 0
    };
    let y = if let Message::Move { x, y } = m2 {
        y
    } else {
        0
    };

    enum Test {
        T(u32),
    }
}
