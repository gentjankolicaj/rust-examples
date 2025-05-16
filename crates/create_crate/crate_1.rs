//declare existing external module
//the contents of the module files would be inserted in places where mod declarations are.
mod modules;

fn main() {
    println!("Hello from crate_1.rs");

    //call public functions on module files;

    modules::a_module::print_info();
    modules::b_module::print_info();
}
