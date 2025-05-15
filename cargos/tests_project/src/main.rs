mod lib;

fn main() {
    let mono = lib::func_lib::Mono::new(10);
    let di=lib::func_lib::Di::new(11,22);
    println!("mono={:?}", mono);
    println!("di={:?}", di);
}
