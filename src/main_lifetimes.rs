fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(PartialEq, PartialOrd)]
struct S1{
    f1: i32,
}

fn longest2(x: &S1, y: &S1) -> S1 {
    S1 {
        f1: 42,
    }
}

fn longest3(x: &i32, y: &i32) -> i32 {
    *x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    let v = "";

    // &s[..]
    v
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}