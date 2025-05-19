//1.In rust-lang <T> declaration implies that T is generic type.

#![allow(dead_code)]
#![allow(unused_variables)]


#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B(A);


#[derive(Debug)]
struct C{
    a: A,
    b: B,
}

fn print_a(a: A){
    println!("Variable of type A:{:?}",a)
}

fn print_b(b: B){
    println!("Variable of type B:{:?}",b)
}

fn print_c(c: C){
    println!("Variable of type C:{:?}",c)
}

//==================================================================================================
//Below declared structs with generics involved.
//==================================================================================================

//tuple-like struct with generic
struct D<T>(T);

//named-field struct with generics
struct E<T,U>{
    t:T,
    u:U,
}


fn main(){
    //instantiation of non-generic structs 
    let a = A;
    let b = B(A);
    let c = C{a:A,b:B(A)};
    
    print_a(a);
    print_b(b);
    print_c(c);
    
    //instantiation of generic structs.
    
    
}