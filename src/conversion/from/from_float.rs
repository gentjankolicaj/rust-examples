//1.In rust, we implement trait 'From' in order to convert a custom type A to custom type B.
//2.Example : let b:B =B.from(a); where let a:A=value;

#[derive(Debug)]
struct Float{
    value:f64,
}

impl From<f32> for Float{
    fn from(param:f32)->Float{
        return Float{value:(param as f64)};
    }
}

impl From<f64> for Float{
    fn from(param:f64)->Float{
        return Float{value:param};
    }
}

fn main(){
    let a=1f32;
    let b=2f64;

    //Float instances from f32 & f64
    let fa=Float::from(a);
    let fb=Float::from(b);
    println!("a={:?},b={:?},fa={:?},fb={:?}", a, b, fa,fb);
}