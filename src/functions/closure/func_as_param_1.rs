static TWO: i32 = 2;
fn apply_void<F: Fn()>(f: F) {
    f();
    println!("apply_void finished.");
}

fn apply_i32<F: Fn(i32)>(f: F, x: i32) {
    f(x);
    println!("apply_i32 finished.");
}

fn void_func() {
    println!("void_func () called.");
}
fn i32_func(i: i32) {
    let product = i * TWO;
    println!("i32_func, product = {}", product);
}

fn main() {
    let void_closure = || {
        println!("void_closure called");
    };
    let i32_closure = |x| {
        let sum = x + TWO;
        println!("i32_closure called, sum={}", sum);
    };

    //apply void function & closure
    apply_void(void_func);
    apply_void(void_closure);

    //apply i32 function & closure
    apply_i32(i32_func, 3);
    apply_i32(i32_closure, 3);
}
