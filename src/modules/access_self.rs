//1.In rust-lang the `self` keyword refers to the current scope.

mod outer {

    fn info() {
        println!("info() from module 'A'");
    }

    mod inner {

        //make function public only to module A
        pub(super) fn info() {
            println!("info() from module 'B'");
        }
    }

    pub fn call() {
        // Calling `self::info()` and calling `info()` directly both give the same result, because they refer to the same function.
        self::info();
        info();
        inner::info();
    }
}

fn main() {
    //call public function of module A
    outer::call();
}
