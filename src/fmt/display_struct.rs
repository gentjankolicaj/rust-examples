use std::fmt;

//define a structure for which fmt::Display will be implemented

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let structure = Structure(101);

    //print structure after implementing fmt::Display
    println!("{}", structure);
}