fn main() {
    let x = 5;
    println!("{:?}", &x as *const i32);
    
    let x = x + 1;
    println!("{:?}", std::ptr::addr_of!(x));
    
    let x = x * 2;
    println!("{:?}", std::ptr::addr_of!(x));

    println!("The value of x is: {}", x);
}
