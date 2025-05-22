//1.In rust-lang <T> declaration implies that T is generic type.

//set file level scope attribute
#![warn(dead_code)]

//Define 2 traits with generics

trait Mono<T> {
    fn value(self) -> T;
}

//Implement trait with generic Mono<T> for Generic
impl<Type> Mono<Type> for Type {
    fn value(self) -> Type {
        self
    }
}

fn main() {
    //define struct a.
    struct A;

    let a0 = A;

    //call trait Mono, method 'value()' of a generic type
    a0.value();
}
