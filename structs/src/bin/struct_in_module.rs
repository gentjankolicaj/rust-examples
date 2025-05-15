pub mod functional {

    #[derive(Debug)]
    pub struct Mono<T> {
        pub value: T,
    }

    #[derive(Debug)]
    pub struct Bi<T, U> {
        pub fist: T,
        pub second: U,
    }

    #[derive(Debug)]
    pub struct Tri<T, U, V> {
        pub fist: T,
        pub second: U,
        pub third: V,
    }

    impl<T> Mono<T> {
        pub fn new(value: T) -> Mono<T> {
            Mono { value }
        }
    }

    impl<T, U> Bi<T, U> {
        pub fn new(fist: T, second: U) -> Bi<T, U> {
            Bi { fist, second }
        }
    }

    impl<T, U, V> Tri<T, U, V> {
        pub fn new(fist: T, second: U, third: V) -> Tri<T, U, V> {
            Tri {
                fist,
                second,
                third,
            }
        }
    }
}

fn main() {
    //instantiate from module
    let mono = functional::Mono::new(1);
    let bi = functional::Bi::new(1, 2);
    let tri = functional::Tri::new(1, 2, 3);

    println!("{:?}", mono);
    println!("{:?}", bi);
    println!("{:?}", tri);
}
