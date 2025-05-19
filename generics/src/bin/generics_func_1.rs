//1.In rust-lang <T> declaration implies that T is generic type.

#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B(A);

#[derive(Debug)]
struct C {
    a: A,
    b: B,
}

fn print_a(a: A) {
    println!("Variable of type A:{:?}", a)
}

fn print_b(b: B) {
    println!("Variable of type B:{:?}", b)
}

fn print_c(c: C) {
    println!("Variable of type C:{:?}", c)
}

//==================================================================================================
//Below declared structs with generics involved.
//==================================================================================================

//tuple-like struct with generic
#[derive(Debug)]
struct D<T>(T);

//named-field struct with generics
#[derive(Debug)]
struct E<T, U> {
    t: T,
    u: U,
}

//explicitly given type i32 because there is no generic specified for function 'print_d'
fn print_d(d: D<i32>) {
    println!("Variable of type D:{:?},D with explicit type i32", d);
}

//generics specified => D doesn't need explicit type
fn print_d_generic<T: std::fmt::Debug>(d: D<T>) {
    println!("Variable of type D:{:?},D with generic type", d);
}

//explicit type give i32 because no generics specified for 'print_e'
fn print_e(e: E<i32, i32>) {
    println!("Variable of type E:{:?},E with explicit type i32", e);
}

//generics specified => D doesn't need explicit type
fn print_e_generic<T: std::fmt::Debug, U: std::fmt::Debug>(e: E<T, U>) {
    println!("Variable of type E:{:?},D with generic type", e);
}

fn main() {
    //instantiation of non-generic structs
    let a = A;
    let b = B(A);
    let c = C { a: A, b: B(A) };

    print_a(a);
    print_b(b);
    print_c(c);

    //instantiation of generic structs.
    let d_i32 = D(0);
    let e_i32 = E { t: 0, u: 0 };

    print_d(d_i32);
    print_d_generic(D(1));

    print_e(e_i32);
    print_e_generic(E { t: 1, u: 1 });
}
