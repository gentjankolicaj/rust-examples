

fn  print() {
    println!("binary: dead_code_attribute, print()");
}

#[allow(dead_code)]
fn  print2() {
    println!("binary: dead_code_attribute, print2(),this function is not used");
}

fn main(){
    print();
}