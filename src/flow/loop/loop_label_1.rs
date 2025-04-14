#![allow(unreachable_code, unused_labels)]
fn main() {
    let mut counter = 0;
    'one: loop {
        println!("Entered the one loop");
        counter += 1;

        'two: loop {
            println!("Entered the two loop");
            // This breaks the two loop
            break 'two;
        }

        'three: loop {
            println!("Entered the three loop");
            break 'three;
        }

        println!("Exiting the one loop, iteration {}", counter);
        if counter < 100 {
            //skip next steps of iteration and continue to new iteration
            continue 'one;
        }

        //definitely break 'one loop
        break 'one;
    }

    println!("Exited the one loop");
}
