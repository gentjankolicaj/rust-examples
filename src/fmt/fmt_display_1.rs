use std::fmt;


//define structure with non-nameable fields
//Drive Debug
#[derive(Debug)]
struct MinMax(i64, i64);

//Implement fmt::Display for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} - {}]", self.0, self.1)
    }
}


// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let min_max = MinMax(-101, 101);

    //print with debug & display
    println!("min-max display:{}", min_max);
    println!("min-max debug: {:?}", min_max);


    //a
    let a = Point2D { x: -99.0, y: 99.0 };
    println!("a display:{}", a);
    println!("a debug: {:?}", a);

    //b
    let b = Point2D { x: -3.14, y: 3.14 };
    println!("b display:{}", b);
    println!("c display:{:?}", b);
}