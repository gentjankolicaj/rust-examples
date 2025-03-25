// not printable
//struct NotPrintable(i32);

#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct Deep(Printable);

fn main() {

    //print with macro
    println!("Printable structure: {:?}", Printable(101));
    println!("Deep structure: {:?}", Deep(Printable(314)));
}