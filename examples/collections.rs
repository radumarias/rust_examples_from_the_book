use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::Hash;
use crate::mod1::mod11::Mod11;

pub mod mod1;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("test".to_string(), 1i32);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    let mut e = &mut v[0];
    println!("{}", e);
    *e = 2;
    println!("{}", v[0]);

    let e2 = v.get(0);
    let e2_v = match e2 {
        Some(v) => v,
        None => &0
    };

    let e3 = v[0];
    v.push(2);
    println!("{}", e3);

    let v2 = vec![Mod11 { s: 1 }];
    let v21 = v2[0];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }

    enum V {
        V1(i32),
        V2(String),
    }
    let v3 = vec![V::V1(1), V::V2("test".to_string())];
    for i in &v3 {
        match i {
            V::V1(v) => println!("{}", v),
            V::V2(v) => println!("{}", v),
        }
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut m = BTreeMap::new();
    for word in text.chars() {
        let count = m.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", m);

    #[derive(Debug, Eq, PartialEq)]
    struct Entry {
        key: String,
        value: i32,
    }
    impl PartialOrd<Self> for Entry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Entry {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value.cmp(&other.value)
        }
    }

    let mut chars_by_nums : Vec<(&String, &i32)> = m.iter().collect();
    chars_by_nums.sort_by(|a, b| b.1.cmp(a.1));
    for (key, value) in chars_by_nums {
        println!("{key}: {value}");
    }
}