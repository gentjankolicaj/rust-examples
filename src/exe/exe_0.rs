fn exe_info() {
    let exe = std::env::current_exe();
    println!("Executing : {:?}", exe);
}
fn main() {
    exe_info();
}
