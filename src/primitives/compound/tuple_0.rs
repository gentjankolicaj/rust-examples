

fn main(){
    //declaration & initialization
    //tuple with inferred types
    let inferred=(true,10u32,10i32);
    println!("tuple with inferred type values: {:?}",inferred);

    //declaration & initialization
    //tuple with annotated types
    let annotated:(bool,i32,bool,f32)= (true,10,false,10.0);
    println!("tuple with annotated type values: {:?}",annotated);
}