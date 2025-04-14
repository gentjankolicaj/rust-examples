//1.In rust-lang we use 'loop' keyword for infinite loop
//2.We can exit loop anytime using 'break' or 'continue'
//3.'break' statement exits 'loop' definitely
//4.'continue' statement skips rest of iteration and continues a new iteration.

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}
