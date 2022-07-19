fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let condition = true;

    let num = if condition {10} else {12};
    println!("{}",num);
}
