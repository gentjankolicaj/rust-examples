

//In rust-lang () type, has exactly one possible value
fn output_empty()->(){
    println!("In rust-lang we can type '()' which is empty.");
    println!("In rust-lang () type, has exactly one possible value");

    () //return value of function
}


fn main(){
    let immutable_empty = output_empty();
    println!("{:?}",immutable_empty);
}