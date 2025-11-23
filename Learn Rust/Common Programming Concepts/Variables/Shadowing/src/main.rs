fn main() {
    let x = 5;
    println!("{}",x);

    // Break it down:
    //     &x = a reference to x.
    // Think: “a safe pointer managed by Rust” (borrowed reference).
    // as *const i32 = cast that reference into a raw pointer:
    // Now it’s an unsafe pointer (like in C), with no borrow checking.
    // You’re converting a safe reference into a raw pointer to see the memory address of x.
    // {:?} = Debug format. For raw pointers, it prints something like:
    println!("{:?}", &x as *const i32);   // old way

    // This uses the raw reference syntax:
    //     &raw const x creates a raw pointer directly.
    //     Type: *const i32 (same as above).
    // No intermediate &x reference is created; it goes straight to a raw pointer.
    // Why does this syntax exist?
    // It avoids creating a temporary reference &x that must follow borrow rules.
    //     It’s safer in low-level scenarios (e.g. with uninitialized memory, fields inside maybe-uninit types, etc.).
    //     For everyday Rust, you can think:
    // “&raw const x is just another way to get a raw pointer to x.”
    println!("{:?}", &raw const x);  // newer, more explicit way
    println!("{:?}", std::ptr::addr_of!(x));

    let x = x + 1; // shadowing
    println!("{}",x);
    println!("{:?}", std::ptr::addr_of!(x));
    
    let x = x * 2; // shadowing
    println!("{}",x);
    println!("{:?}", std::ptr::addr_of!(x));

    println!("The value of x is: {}", x);
}
