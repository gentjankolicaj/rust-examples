
//In this sample I perform:
//1. Take a negative signed int
//2. Get absolute value using shift , XOR , subtract operations.

//Refer to : https://stackoverflow.com/questions/12041632/how-to-compute-the-integer-absolute-value

fn main(){
    let  mut x:i32=-911;
    let mut mask:i32=x>>31;
    println!("mask:{}",mask);

    x= x ^ mask;
    println!("x ^ mask:{}",x);

    x=x-mask;
    println!("x-mask:{}",x);
}