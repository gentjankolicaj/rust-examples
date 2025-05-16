

#[derive(Debug)]
struct Pair(i32,i32);


fn main(){
    //declare immutable variable that holds address value of type Pair
    let pair = Pair(1,2);

    println!("debug-attribute: pair={:?}",pair);
}