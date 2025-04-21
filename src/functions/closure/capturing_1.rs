/*
- Closures can capture variables:
1.by reference: &T
2.by mutable reference: &mut T
3.by value: T
*/

#[derive(Debug)]
enum N {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
}
fn main() {
    let chosen = N::ONE;

    //capturing by reference '&' chosen and borrow reference.
    //It will remain borrowed until `closure` is used the last time.
    let closure = || {
        println!("Enum N chosen {:?}", chosen);
    };

    closure();

    //`chosen` can be borrowed immutably again, because the closure only holds an immutable reference to `chosen`.
    let _reborrowed = &chosen;
    println!("printing _reborrowed {:?}", _reborrowed);

    closure();
}
