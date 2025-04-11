//1.In rust-lang we use 'loop' keyword for infinite loop
//2.We can exit loop anytime using 'break' or 'continue'
//3.'break' statement exits 'loop' definitely
//4.'continue' statement skips rest of iteration and continues a new iteration.

fn main() {
    let mut counter: i32 = 0;

    // Infinite loop
    loop {
        println!("{}", counter);
        counter = counter + 1;

        if counter > 1_0_000 {
            //breaks infinite loop
            break;
        }
    }
    println!("OUTSIDE-LOOP !!!");
}
