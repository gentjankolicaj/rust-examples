//1.In rust-lang we use 'for in' construct to iterate through an Iterator
//2.In rust-lang notation 'a..b' yields values a (inclusive) to b (exclusive)

fn main() {
    //a inclusive to b exclusive
    let a = 0;
    let b = 30;

    for i in a..b {
        //print value from 0 to 29
        println!("{}", i);
    }
}