#![allow(unreachable_code, unused_labels)]
fn main() {
    let mut counter = 0;
    'one: loop {
        println!("Entered the one loop");

        'two: loop {
            println!("Entered the two loop");
            // This breaks the two loop
            break 'two;
        }

        'three: loop {
            println!("Entered the three loop");
            println!("Breaking external parent loop 'one from internal loop 'three");
            break 'one;
        }

        //this line is never reached because of:  break 'one;
        println!("Exiting the one loop, iteration {}", counter);

        //definitely break 'one loop
        break 'one;
    }

    println!("Exited the one loop");
}
