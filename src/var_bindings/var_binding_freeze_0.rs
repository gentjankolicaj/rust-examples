//1.In Rust basically we can freeze a mutable variable binding with (shadowing by immutable & reassignment on same time)
//2.Example : we can shadow variable binding a with  [ let a; or let mut a;]

fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;
        println!("inside block _mutable_integer : {}", _mutable_integer);

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("after block _mutable_integer : {}", _mutable_integer);
}
