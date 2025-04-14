//1.In rust-lang we use 'loop' keyword for infinite loop
//2.We can exit loop anytime using 'break' or 'continue'
//3.'break' statement exits 'loop' definitely
//4.'continue' statement skips rest of iteration and continues a new iteration.

fn main() {
    let mut counter: i32 = 0;

    // Infinite loop
    loop {
        counter = counter + 1;

        //if counter even skip rest of iteration
        if counter % 2 == 0 {
            println!("EVEN: {}", counter);
            continue;
        }
        if counter > 100 {
            break;
        }
        println!("{}", counter);
    }
    println!("OUTSIDE-LOOP !!!");
}
