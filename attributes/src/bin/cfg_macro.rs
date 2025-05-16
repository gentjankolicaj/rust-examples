
fn main(){

    //call cfg! macro

    if cfg!(target_os = "linux") {
        println!("Hello, Linux!");
    }else{
        println!("Hello, NonLinux!");
    }
}