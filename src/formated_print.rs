fn main() {
    println!("Formated print {}", 10);
    println!("First:{0} , second:{1}", 3.14, "John doe");
    println!("Bases: ");

    //Bases print
    println!("Base 2 {:b}", 16);
    println!("Base 8 {:o}", 16);
    println!("Base 10 {}", 16);
    println!("Base 16 {:x}", 16);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // rust-lang even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
