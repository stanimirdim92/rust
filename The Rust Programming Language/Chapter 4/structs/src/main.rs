
#[derive(Debug,  Clone)]
struct Phone {
    code: u16,
    number: u64,
    description: String
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: Option<u8>,
    email: String,
    phone: Phone
}

impl  Person {
    fn details(&self, other: &Self) {
        println!("{:?} 4", self);
        println!("{:?} 5", other);
    }
}

fn main() {
     let  phone = Phone {
         code: 359,
         number: 123456789111,
         description: "".to_string(),
     };
    let person = Person{
        name: "Jane".parse::<String>().unwrap(),
        age: None,
        email: String::from( "test@gmail.com"),
        phone
    };
    let mut person2 = person.clone();
    person2.name = String::from("John");
    person2.age =Some(31);

    println!("{:?} 1", person);
    println!("{} 2 ", person.name);
    println!("{} 222 ", person2.age.unwrap_or( 0)); // See below comment
    println!("{:?} 3", person.details(&person2));

    match person2.age {
        Some(p) => println!("has value {p}"),
        None => println!("has no value"),
    }


}


//
// In Rust, both to_owned().unwrap() and unwrap().to_owned() are valid sequences of method calls,
// but their applicability and behavior depend on the context,
// specifically the type of the value you're working with and what you're trying to achieve.
//
// to_owned().unwrap(): This sequence implies that to_owned() is called first,
// potentially converting a borrowed type (like a &str or a &[T])
// into its owned counterpart (like a String or a Vec<T>).
// The unwrap() call is then applied to the result of to_owned().
// This sequence makes sense if to_owned() returns a Result or Option type,
// which is not typically the case for to_owned()
// but might be for similar methods that return a Result or Option.
//
// unwrap().to_owned(): Here, unwrap() is called on a Result or Option,
// presumably to extract the value contained within,
// panicking if the Result is an Err or the Option is None.
// After successfully unwrapping, to_owned() is called on the unwrapped value.
// This sequence is useful when you have a borrowed value wrapped in a Result or Option
// and you want to both extract and convert it to an owned value.
//
// Practical Difference:
// to_owned().unwrap() might not be as commonly used or make less sense in typical Rust code,
// because to_owned() is usually applied to borrowed data to create an owned instance
// and does not return a Result or Option that needs unwrapping.
//
// unwrap().to_owned() is more practical when dealing with a Result or Option containing a reference
// that you want to convert into an owned version.
// For example, if you have an Option<&str> and you want a String,
// or if you're dealing with Result<&T, E> and want an owned T.

