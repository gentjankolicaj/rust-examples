//1.In rust-lang we use mod 'module' to bring items (modules/func/traits/structs/) to scope.

//2.Start by declaring mod persistence and order of search is :
//2.a.Look for a file 'persistence.rs' in current directory
//2.b.If file not found, look at directory 'persistence' for file mod.rs

mod persistence;

fn main() {
    persistence::create_users();
    persistence::create_addresses();
}
