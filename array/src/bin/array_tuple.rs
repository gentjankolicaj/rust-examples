use std::mem::{align_of_val, size_of_val};


fn main(){
    let at:[(bool,i32);2] = [(false,0), (true,2)];

    println!("at={:?}", at);
    println!("at size_of_val={}", size_of_val(&at));
    println!("at align_of_val={}", align_of_val(&at));
}