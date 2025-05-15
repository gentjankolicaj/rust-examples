pub mod func_lib {

    #[derive(Debug)]
    pub struct Mono<T> {
        pub t: T,
    }

    #[derive(Debug)]
    pub struct Di<T, U> {
        pub t: T,
        pub u: U,
    }

    impl<T> Mono<T> {
        pub fn new(t: T) -> Mono<T> {
            Mono { t }
        }
    }

    impl<T, U> Di<T, U> {
        pub fn new(t: T, u: U) -> Di<T, U> {
            Di { t, u }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    #[test]
    fn mono_test() {
        assert_not_eq!(Mono::new(10), Mono::new(10));
    }

    #[test]
    fn di_test() {
        assert_not_eq!(Di::new(10,0), Di::new(10,0));
    }

}

