use std::ffi::{OsStr, OsString};

#[warn(dead_code)]
#[warn(unused_variables)]
fn main() {

    let osstr1:&OsStr =  OsStr::new("Mary had a little lamb");
    let osstr2:&OsStr =  OsStr::new("Mary didn't had a little lamb");
    let parsed:OsString = concat_os_strings(osstr1, osstr2);

    println!("{}",str_len(osstr1.to_os_string()));

    println!("{}",osstr1.to_os_string().into_string().unwrap());
    println!("{}, {}",parsed.clone().into_string().unwrap(),parsed.len());
}
fn concat_os_strings(a: &OsStr, b: &OsStr) -> OsString {
    let mut ret = OsString::with_capacity(a.len() + b.len()); // This will allocate
    ret.push(a); // This will not allocate further
    ret.push(b); // This will not allocate further
    ret
}


// usize, because len is always positive or zero
// we can set s as a reference saving us some memory and lines
fn str_len(s: OsString) -> usize {
    let a = s.len();
    a
}