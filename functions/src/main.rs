fn main() {
    println!("Hello, world!");
    another_fn(5);
    print_labeled_measurement(10,'h');

    //block
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let res = five();
    println!("{}",res);
    //increment res
    println!("{}",plus_one(res));
}


fn another_fn(x:i32) {
    println!("the value of x is:{}",x);
}

fn print_labeled_measurement(value:i32,unit_label:char) {
    println!("The measurement is:{}{}",value,unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x:i32)-> i32 {
    x + 1
}