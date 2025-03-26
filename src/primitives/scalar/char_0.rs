fn main() {
    //char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    //=> 4 bytes * 2^8 => 2^32
    let infinity: char = '∞';

    //
    let a: char = 'a';
    let a1: char = '1';

    println!("infinity={}", infinity);
    println!("a={}", a);
    println!("a1={}", a1);
}
