static TWO: i32 = 2;

fn apply_i32<F: Fn(i32) -> i32>(f: F, x: i32) {
    let val = f(x);
    println!("apply_i32 finished with value={}.", val);
}

fn i32_func(i: i32) -> i32 {
    let product = i * TWO;
    println!("i32_func, product = {}", product);

    product //return product
}

fn main() {
    let i32_closure = |x| {
        let sum = x + TWO;
        println!("i32_closure called, sum={}", sum);

        sum //return sum;
    };

    //apply i32 function & closure
    apply_i32(i32_func, 3);
    apply_i32(i32_closure, 3);
}
