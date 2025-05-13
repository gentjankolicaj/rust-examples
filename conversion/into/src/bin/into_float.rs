//1.In rust-lang we have interchangeable traits 'From' and 'Into' that we need to implement in order to convert custom types.
//2.For primitive types however conversion is done with casting.

#[derive(Debug)]
struct Float {
    value: f64,
}

impl Into<Float> for f32 {
    fn into(self) -> Float {
        return Float {
            value: (self as f64),
        };
    }
}

impl Into<Float> for f64 {
    fn into(self) -> Float {
        return Float { value: self };
    }
}

fn main() {
    //Primitive variable bindings
    let a = 1f32;
    let b = 2f64;

    //Convert from primitive into custom-type Float
    let fa: Float = a.into();
    let fb: Float = b.into();
    println!("a={:?},b={:?},fa={:?},fb={:?}", a, b, fa, fb);
}
