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

//==================================================================================================
//Below declared structs with generics involved.

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
    
    //instantiation of generic structs.
    
    
}