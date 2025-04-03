//1.In Rust basically we can freeze a mutable variable binding with (shadowing by immutable & reassignment on same time)
//2.Example : we can shadow variable binding a with  [ let a; or let mut a;]

fn main() {
    let mut mutable_var_bind = 101;

    {
        //Shadow by immutable variable binding  & freeze binding
        let mutable_var_bind = mutable_var_bind;
        println!(
            "mutable_var_bind shadowed by immutable & bound : {}",
            mutable_var_bind
        );
    }

    println!("after first-block shadow & freeze : {}", mutable_var_bind);

    {
        //Shadow by mutable variable binding, in this case we don't have freeze.
        let mut mutable_var_bind = mutable_var_bind;
        println!(
            "mutable_var_bind shadowed by mutable & bound : {}",
            mutable_var_bind
        );
    }

    println!("after first-block shadow & freeze : {}", mutable_var_bind);
}
