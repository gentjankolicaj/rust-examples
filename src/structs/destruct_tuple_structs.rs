#[derive(Debug)]
struct Pair(i32, i32);

fn main() {
    let a = Pair(10, 20);
    //destruct tuple structure
    let Pair(a_left, a_right) = a;
    println!("structure Pair: {:#?} ,a_left:{:?},a_right:{:?}", a, a_left, a_right);
}