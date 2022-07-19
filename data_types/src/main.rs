fn main() {
    let guess: u32 = "42".parse().expect("Not a Number");
    println!("{}",guess);
    let x = 2.0;
    let y:f32 = 3.0;
    println!("{},{}",x,y);
     // addition
     let sum = 5 + 10;
     println!("sum is:{}",sum);
     // subtraction
     let difference = 95.5 - 4.3;
     println!("difference is:{}",difference);
     // multiplication
     let product = 4 * 30;
     println!("product is:{}",product);
 
     // division
     let quotient = 56.7 / 32.2;
     println!("quotient is:{}",quotient);
     let floored = 2 / 3; // Results in 0
     println!("floored is:{}",floored);
     // remainder
     let remainder = 43 % 5;
     println!("remainder is:{}",remainder);
     let t = true;
     let f:bool = false;
     let c = 'z';
    //  let z = 'Z';
     
     //compound types
     let tup:(i32,f64,i8) = (500,6.4,1);
     let (x,y,z) = tup;

     println!("value of x is:{}",tup.0);
     println!("value of y is:{}",tup.1);
     println!("value of z is:{}",tup.2);

     let a = [1,2,3,4,5,6,7];
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

//array declaration by specifying the type and size.
let a: [i32; 5] = [1, 2, 3, 4, 5];
//array declaration and assigning a single value to every element of the array
let a = [3; 5];
//accessing array elements 
println!("{}",a[0]);
}
