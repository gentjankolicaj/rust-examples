//1.In rust-lang <T> declaration implies that T is generic type.

//set file level scope attribute
#![warn(dead_code)]

//define 3 types of structs & implement a trait with generics
#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B(A);

#[derive(Debug)]
struct C {
    a: A,
    b: B,
}

//Define 2 traits with generics
trait Mono<T> {
    fn value(self) -> T;
}

trait Di<T, U> {
    fn first(self) -> T;
    fn second(self) -> U;
}

//Implement trait Mono<T> for structs with concrete types
impl Mono<A> for A {
    fn value(self) -> A {
        println!("a = {:?}", self);
        self
    }
}

impl Mono<B> for B {
    fn value(self) -> B {
        println!("b = {:?}", self);
        self
    }
}

impl Mono<C> for C {
    fn value(self) -> C {
        println!("c = {:?}", self);
        self
    }
}

//Implement trait Di<T,U> with concrete type
impl Di<A, B> for C {
    fn first(self) -> A {
        println!("Di a = {:?}", self.a);
        self.a
    }

    fn second(self) -> B {
        println!("Di b = {:?}", self.b);
        self.b
    }
}

fn main() {
    let a = A;
    let b = B(A);
    let c = C { a: A, b: B(A) };

    //call implemented traits defined with generics & implemented with concretes
    a.value();
    b.value();
    c.value();

    //call implemented trait Di with concrete types A,B
    let c2 = C { a: A, b: B(A) };
    c2.first();
    //  c2.second(); compile error about using it.
}
