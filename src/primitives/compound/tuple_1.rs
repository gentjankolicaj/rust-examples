

fn main(){
    //declaration
    //initialization separately
    //tuple with annotated types
    let inferred:(bool,bool);
    inferred=(true,false);
    println!("tuple (declaration * initialization separately) with annotated type values: {:?}",inferred);

    //declaration
    //initialization separately
    //tuple with annotated types
    let annotated:(bool,i32,bool,f32);
    annotated=(true,1,false,3.14);
    println!("tuple (declaration * initialization separately) with annotated type values: {:?}",annotated);
}