#[allow(unused)]
fn main() {

    let var = 5;
    let var_typed: i32 = 5;
    let mut mut_vat = 5;
    let shadowed_var = 5;
    let shadowed_var_typed: f32 = 13.37;

    println!("shadowed_var before override  is {}", shadowed_var); // 5
    let shadowed_var: i8 = 5 + 1;
    println!("shadowed_var after override is {}", shadowed_var); // 6

    {
        let shadowed_var: i8 = shadowed_var + 6;
        println!("shadowed_var inside block after being shadowed is {}", shadowed_var); // 12
    }
    println!("shadowed_var outside after being shadowed is {}", shadowed_var); // 6

    // not working: calls in constants are limited to constant functions, tuple structs and tuple variants
    // const STRING: String = String::from("this is a string");
    // const STRING_REPEATED: String = "abc".repeat(4);
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");
    const STRING_SLICE: &str = "this is a string";
    const ONE_DAY: i32 = 60*60*24;
    const ONE_TUPLE: (i32, i32, &str) = (1, 2, "3");
    const ONE_ARRAY: [i32; 3] = [1,2,3];
    // |i| is a clojure
    ONE_ARRAY.iter().for_each(move |i:&i32| {
        println!("{i}");
    });

    let s = String::from("hello");
    fn takes_ownership(some_string: &String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    // & is a reference
    // referencia, prepratka
    takes_ownership(&s);
    println!("{s}1");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    // BIG PROBLEM if assigned before s1,s2 are used
    // using them afterward is also not valid

    // println!("{}, {}", r1, r2); // will throw an error
}
