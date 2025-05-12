extern crate funcs;

fn main() {
    let mono = funcs::Mono::new(32);
    let di = funcs::Di::new(10, 3.14);
    let tri = funcs::Tri::new(true, 101, ());

    println!("mono = {:?}", mono);
    println!("bi = {:?}", di);
    println!("tri = {:?}", tri);
}
