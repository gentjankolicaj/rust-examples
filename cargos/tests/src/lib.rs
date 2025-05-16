pub mod funcs {

    #[derive(Debug, PartialEq)]
    pub struct Mono<T> {
        pub t: T,
    }

    #[derive(Debug, PartialEq)]
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

    #[cfg(test)]
    mod unit_tests {
        use crate::funcs::{Di, Mono};

        #[test]
        fn mono_test() {
            assert_ne!(Mono::new(10), Mono::new(11));
        }

        #[test]
        fn di_test() {
            assert_ne!(Di::new(10, 0), Di::new(10, 1));
        }
    }
}
