//1.In rust-lang the `super` keyword refers to the parent scope.

mod outer {

    fn info() {
        println!("info() from module 'A'");
    }

    mod inner {

        //make function public only to module A with super keyword
        pub(super) fn info() {
            println!("info() from module 'B'");
        }
    }

    pub fn call() {
        // Calling `self::info()` and calling `info()` directly both give the same result, because they refer to the same function.
        println!("pub call() from module 'A'");
        self::info();
        info();
        inner::info();
    }

    //make this function accessible only to parent scope
    pub(super) fn call2() {
        println!("pub(super) call2() from module 'A'");
    }
}

fn main() {
    //call public function of module A
    outer::call();
    outer::call2();
}
