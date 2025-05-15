

use tests_project::funcs::Di;
use tests_project::funcs::Mono;


fn main() {
    let mono = Mono::new(10);
    let di = Di::new(11, 22);
    println!("mono={:?}", mono);
    println!("di={:?}", di);
}
