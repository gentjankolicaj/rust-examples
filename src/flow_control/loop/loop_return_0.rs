fn main() {
    let mut counter = 0;

    let result = 'one: loop {
        if counter > 100 {
            break counter;
        }
        counter += 1;
    };

    println!("Loop 'one iterated {} times", result);
}