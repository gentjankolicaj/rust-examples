
#[allow(dead_code)]

//Define unit-struct T
#[derive(Debug)]
struct T;

//Define tuple-struct Mono(T)
#[derive(Debug)]
struct Mono(T);

//Define named-field-struct Di
#[derive(Debug)]
struct Di{
    first:T,
    second: Mono,
}

//Declare a generic function print_gen
//Note: that function param is generic type param <A> and A is generic, not of type struct T.

fn generic_print<A: std::fmt::Debug>(arg:A){
    println!("{:?}",arg)
}

fn main(){
    
    //instantiated T
    let unit=T;
    
    //instantiated T and passed address value at Mono which is then instantiated
    let tuple=Mono(T);
    
    //Inside {} instantiated 2 times T
    let named=Di{first:T, second:Mono(T)};
    
    generic_print(unit);
    generic_print(tuple);
    generic_print(named);
}

