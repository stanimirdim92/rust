use std::ops::Range;

#[warn(dead_code)]
#[warn(unused_variables)]
fn main() {
    let mut s = String::from("hello");
    let word = first_word(&s);

    s.clear();
    println!("{}", word);
    println!("=====");

    let mut s = String::from("hello word");
    let word2 = second_word(&s);
    println!("{}", word2);
}


fn first_word(s: &String) -> usize {
    let str  = s.as_bytes();

    for (i, &item) in str.iter().enumerate() {
        println!("{:?} {}", item,i);
        if i == 3 {
            return i
        }
    }

    s.len()
}
fn second_word(s: &String) -> &str {
    let str  = s.as_bytes();

    for (i, &item) in str.iter().enumerate() {
        println!("{:?} {} {}", item, &item,i);
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}