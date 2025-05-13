//1.In rust-lang the 'for in' construct is able to interact with an Iterator in several ways.
//2.In rust-lang by default the for loop will apply the 'into_iter' function to the collection.
//3.In rust-lang 'into_iter' function consumes the collection so that on each iteration the exact data is provided.

fn main() {
    let names = vec!["john", "doe", "jane", "doe", "john117", "master-chief"];

    //clone() because : e: `into_iter` takes ownership of the receiver `self`, which moves `names`
    for name in names.clone().into_iter() {
        match name {
            "john" => println!("John"),
            "doe" => println!("Doe"),
            "jane" => println!("Jane"),
            _ => println!("nothing matched."),
        }
    }

    println!("names: {:?}", names);
}
