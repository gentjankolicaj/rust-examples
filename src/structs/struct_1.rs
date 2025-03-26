#[derive(Debug, Copy, Clone)]
struct User {
    id: i64,
    age: u8,
}

impl User {
    fn new() -> Self {
        Self { id: 0, age: 0 }
    }
}

fn main() {
    let john_doe = User { id: 10, age: 10 };
    println!("john_doe {:#?}", john_doe);

    //new function
    println!("fn User::new() -> {:#?}", User::new());

    //initialized array of structs
    let init_array = [User::new(); 5];
    println!("init_array {:#?}", init_array);
}
