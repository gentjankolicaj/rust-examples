#[derive(Debug, Copy, Clone)]
struct User {
    id: i64,
    age: u8,
}

impl User {
    fn new() -> Self {
        Self { id: 0, age: 0 }
    }
    fn copy_impl(user: User) -> Self {
        return User { id: user.id, age: user.age };
    }
}

fn main() {
    let john_doe = User { id: 10, age: 10 };
    println!("john_doe {:#?}", john_doe);

    //new function
    println!("fn User::new() -> {:#?}", User::new());

    //copied instance of User
    println!("fn User::copy_impl -> {:#?}", User::copy_impl(john_doe));
}
