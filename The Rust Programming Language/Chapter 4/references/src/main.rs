
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

use std::ffi::{OsStr, OsString};

#[warn(dead_code)]
#[warn(unused_variables)]
fn main() {

    let osstr1:&OsStr =  OsStr::new("Mary had a little lamb");
    let osstr2:&OsStr =  OsStr::new("Mary didn't had a little lamb");
    let parsed_osstr:OsString = concat_os_strings(osstr1, osstr2);

    println!("{}",str_len(&osstr1.to_os_string()));

    println!("{}",osstr1.to_os_string().into_string().unwrap());
    println!("{}, {}", parsed_osstr.clone().into_string().unwrap(), parsed_osstr.len());


    let mut s = String::from("hello");

    let ss =  s.clone();
 //   let ss2 = &mut s;
    println!("{}, {}",ss.to_string(), s);
    change(&mut s);

    let _reference_to_nothing = dangle();
}
fn concat_os_strings(a: &OsStr, b: &OsStr) -> OsString {
    let mut ret = OsString::with_capacity(a.len() + b.len()); // This will allocate
    ret.push(a); // This will not allocate further
    ret.push(b); // This will not allocate further
    ret
}


// usize, because len is always positive or zero
// we can set s as a reference saving us some memory and lines
fn str_len(s: &OsString) -> usize {
    s.len()
    // let a = s.len();
    // a
}

// string ownership
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// adding reference operator (&) will =>
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime
fn dangle() -> String {
    let s = String::from("hello");

    s
}