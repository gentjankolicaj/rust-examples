//struct sample with generic field

#[derive(Debug)]
struct Wrapper<T> {
    data: T,
}

#[derive(Debug)]
struct Wrapper2<T, U> {
    data: T,
    data2: U,
}

impl<T> Wrapper<T> {
    pub fn new(data: T) -> Self {
        return Self { data };
    }
}

impl<T, U> Wrapper2<T, U> {
    pub fn new(data: T, data2: U) -> Self {
        return Self { data, data2 };
    }
}

fn main() {
    //first instantiation approach, with fields
    let a = Wrapper { data: 10 };
    let b = Wrapper2 {
        data: 10.0,
        data2: 20.0,
    };
    println!("a={:?},b={:?}", a, b);

    //second approach with function new()
    let c = Wrapper::new(10);
    let d = Wrapper2::new(10.0, 20.0);
    println!("c={:?},d={:?}", c, d);
}
