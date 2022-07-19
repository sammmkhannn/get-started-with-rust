fn main() {
    let mut x:u32 = 5;
    println!("value of x is:{}",x);
    x = 6;
    println!("value of x is:{}",x);
    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;
    //shadowing
    let y:u32 = 40;
    let y = 60;
    let spaces = "    ";
    let spaces = spaces.len();

}
