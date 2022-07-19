fn main() {
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);


    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);
    let mut counter:i32 = 10;
    while counter != 0 {
        println!("{}",counter);
        counter -=1;
    }
    let a = [10,20,30,40,50];
    for element in a {
        println!("{}",element);
    }
    for number in (1..4).rev() {
        println!("{}!",number);
    }
    // let s = String::from("hello");
    // s.push_str(",world!");
    // println!("{}",s);

    //ownership
    let x = 5;
    let y = x;
    //clone method

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1={},s2={}",s1,s2);
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    //passing reference
    let s7 = String::from("hello");
    let len = calculate_length(&s7);

    println!("The length of '{}' is {}.", s1, len);
    
}

fn calculate_length(s:String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
    
        (s, length)
    }
