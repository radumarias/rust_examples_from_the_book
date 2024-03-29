use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let file_r = File::open("hello.txt");
    let mut file = file_r.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
    });

    let mut username = String::new();
    // match file.read_to_string(&mut username) {
    //     Ok(_) => {
    //         println!("Username: {}", username);
    //         Ok(())
    //     }
    //     Err(e) => Err(e),
    // }

    File::open("hello.txt")?.read_to_string(&mut username)?;
    let username = fs::read_to_string("hello.txt")?;

    Ok(())
}

fn last_char_of_first_line(s: &str) -> Option<char> {
    s.lines().next()?.chars().last()
}