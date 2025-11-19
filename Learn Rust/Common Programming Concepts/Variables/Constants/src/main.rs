fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const THREE_HOURS_IN_SECONDS2: [[u8; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

    println!("{}", THREE_HOURS_IN_SECONDS);
    println!("{:?}", THREE_HOURS_IN_SECONDS2)
}
