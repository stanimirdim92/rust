fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const THREE_HOURS_IN_SECONDS2: [[u8; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

    const ASD: &'static str  = "Three hours in";

    let x = 1;
    let x = 2;

    println!("{}", x);
    println!("{}", THREE_HOURS_IN_SECONDS);
    println!("{:?}", THREE_HOURS_IN_SECONDS2);
    println!("{:#}", ASD);
}
