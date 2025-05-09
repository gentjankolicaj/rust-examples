//1.a primitive compound data type in rust
//2.Array is a collection of object of type T.
//3.Array's length is known at compile.
//4.Array BNF: let | let mut identifier (:[T;length]=[t..length] | =[t...length | [t1,t2,t3])

fn main() {
    let a = [1i32, 2i32, 3i32];
    println!("a {:?}", a);

    for i in 0..a.len() {
        println!("for a.len() => a {:?}", a[i]);
    }
    for item in a.iter().enumerate() {
        let pair: (usize, &i32) = item;
        println!("for a.iter().enumerate() => pair={:?}", pair);
    }

    let mut a_mut = [1, 2, 3];
    a_mut[0] = 0;
    println!("a_mut {:?}", a_mut);

    for i in 0..a_mut.len() {
        println!("for a_mut.len() => a_mut {:?}", a_mut[i]);
    }
    for item in a_mut.iter().enumerate() {
        let pair: (usize, &i32) = item;
        println!("for a_mut.iter().enumerate() => pair={:?}", pair);
    }
}
