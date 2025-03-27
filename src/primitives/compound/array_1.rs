//1.A primitive compound data type in rust
//2.Array is a collection of object of type T.
//3.Array's length is known at compile.
//4.Array BNF: let | let mut identifier (:[T;length]=[t..length] | =[t...length | [t1,t2,t3])

fn main() {
    let a = [1i32, 2i32, 3i32];
    println!("a {:?}", a);

    let b: [i8; 7] = [0; 7];
    println!("b {:?}", b);

    let mut a_mut = [1, 2, 3];
    a_mut[0] = 0;
    println!("a_mut {:?}", a_mut);

    let mut b_mut: [f32; 3] = [0.0, 0.0, 1.0];
    b_mut[2] = 3.0;
    println!("b_mut {:?}", b_mut);
}
