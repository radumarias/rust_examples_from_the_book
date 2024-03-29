// use std::cmp::Ordering;
// use std::io;
// use rand::Rng;

// use std::future::pending;

use rand::Rng;

fn main() {
    // println!("Guess the number!");
    //
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    //
    // loop {
    //     println!("Input your guess.");
    //
    //     let mut guess = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line.");
    //
    //     let guess: i32 = match  guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //
    //     println!("You guessed: {guess}");
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    // // println!("{}", s);
    // println!("{}", r3);
    // s.push_str("1");

    // println!("{}", r1);

    #[derive(Debug)]
    struct Rectangle<T> {
        width: i32,
        height: i32,
    }

    impl Rectangle {
        fn area(&self) -> i32 {
            self.width * self.height
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn square(size: i32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..100);

    let r = Rectangle {
        width: 42,
        height: 42,
    };
    let _square = Rectangle::square(42);
    dbg!(&r);
    println!();
    eprintln!("r.width() = {:#?}", r.width());
    println!("area = {}", r.area());


}
