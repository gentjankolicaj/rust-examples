

#[cfg(target_os="linux")]
fn greeting(){
    println!("Welcome to #cfg attribute on linux!");
}

fn main(){
    greeting()
}