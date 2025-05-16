

#[derive(Debug)]
struct Pair(i32,i32);


fn main(){
    //declare immutable variable that holds address value of type Pair
    let pair = Pair(1,2);
    let Pair(first,second) = pair;
    
    println!("debug-attribute: pair={:?}",pair);
    println!("debug-attribute: first={:?}",first);
    println!("debug-attribute: second={:?}",second);
}