//1.In rust-lang 'if-let' is used in place of 'match' sometimes.
//2.This means that 'if let' is used for destructing

fn main() {
    let optional = Some(0);
    let letter: Option<i32> = Some(10);

    //Reads: 'if let' destructures 'optional' into Some(x) then enter block
    if let Some(x) = optional {
        println!("optional destructed by 'if let' , x={}", x);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }
}