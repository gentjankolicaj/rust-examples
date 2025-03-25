#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}


fn main() {
    let name = "John doe";
    let age = 30;
    let person = Person { name, age };

    //print with macro
    println!("first person details: {:#?}", person);

    //print second person
    println!("Second person details: {:#?}", Person { name, age: age + 10 });
}