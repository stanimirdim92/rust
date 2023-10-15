#[allow(unused)]
fn main() {

    let var = 5;
    let var_typed: i32 = 5;
    let mut mut_vat = 5;
    let shadowed_var = 5;
    let shadowed_var_types: u8 = 5;

    println!("shadowed_var before override  is {}", shadowed_var);
    let shadowed_var: i32 = 5 + 1;
    println!("shadowed_var after override is {}", shadowed_var);

    {
        let shadowed_var: i32 = shadowed_var + 6;
        println!("shadowed_var inside after being shadowed is {}", shadowed_var);
    }
    println!("shadowed_var outside after being shadowed is {}", shadowed_var);

    // not working: calls in constants are limited to constant functions, tuple structs and tuple variants
    // const STRING: String = String::from("this is a string");
    // const STRING_REPEATED: String = "abc".repeat(4);

    const STRING_SLICE: &str = "this is a string";
    const ONE_DAY: i32 = 60*60*24;
    const ONE_TUPLE: (i32, i32, &str) = (1, 2, "3");
    const ONE_ARRAY: [i32; 3] = [1,2,3];
    ONE_ARRAY.iter().for_each(move |(i)| {
        println!("{i}")
    });
}
