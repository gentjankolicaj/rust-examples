//1.In rust-lang the 'for in' construct is able to interact with an Iterator in several ways.
//2.In rust-lang by default the for loop will apply the 'into_iter' function to the collection.
//3.In rust-lang 'iter_mut' func mutably borrows each element of the collection, allowing for the collection to be modified in place.

fn main() {
    let mut names = vec!["john", "doe", "jane", "doe", "john117", "master-chief"];

    for name in names.iter_mut() {
        *name = match *name {
            "john" => {
                println!("John");
                "john-updated"
            }
            "doe" => {
                println!("Doe");
                "doe-updated"
            }
            "jane" => {
                println!("Jane");
                "jane-updated"
            }
            _ => {
                println!("nothing matched.");
                "nothing-matched-updated"
            }
        }
    }

    println!("names: {:?}", names);
}
