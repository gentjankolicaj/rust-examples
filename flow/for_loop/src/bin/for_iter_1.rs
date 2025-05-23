//1.In rust-lang the 'for in' construct is able to interact with an Iterator in several ways.
//2.In rust-lang by default the for loop will apply the 'into_iter' function to the collection.
//3.In rust-lang 'iter()' function borrows each element of the collection through each iteration.

fn main() {
    let names = vec!["john", "doe", "jane", "doe", "john117", "master-chief"];

    for name in names.iter() {
        match name {
            &"john" => println!("John"),
            &"doe" => println!("Doe"),
            &"jane" => println!("Jane"),
            _ => println!("nothing matched."),
        }
    }

    println!("names: {:?}", names);
}
