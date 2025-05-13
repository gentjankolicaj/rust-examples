//1.rust-lang has 2 types of constants which can be declared in any scope including global.
//2.Two types of constants are : const & static.
//3.'const' unchangeable value
//4.'static' a possible mutable value with static lifetime.
//5.The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.

//Global constants
const A: f32 = 3.14;
static B: &str = "user";

fn local_consts() {
    const C: i32 = 0;
    static D: i32 = 1;
    println!("Local constants: C= {}, D = {}", C, D);
}

fn main() {
    println!("Global constants : A={},B={}", A, B);
    local_consts();
}
