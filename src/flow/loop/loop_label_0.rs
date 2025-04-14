#![allow(unreachable_code, unused_labels)]
fn main() {
    'one: loop {
        println!("Entered the one loop");

        'two: loop {
            println!("Entered the two loop");
            // This breaks the two loop
            break 'two;
        }

        'three: loop {
            println!("Entered the three loop");
            break 'three;
        }

        println!("Exiting the one loop");
        break 'one;
    }

    println!("Exited the one loop");
}
