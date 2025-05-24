## Generics

- Generics is the topic of generalizing types and functionalities to broader cases.
- This is extremely useful for reducing code duplication in many ways, but can call for rather involved syntax.
- Namely, being generic requires taking great care to specify over which types a generic type is actually considered
  valid.
- The simplest and most common use of generics is for type parameters.
- A type parameter is specified as generic by the use of angle brackets and upper camel case: <Aaa, Bbb, ...>. "Generic
  type parameters" are typically represented as <T>. In Rust, "generic" also describes anything that accepts one or more
  generic type parameters <T>.
- Any type specified as a generic type parameter is generic, and everything else is concrete (non-generic).
- For example, defining a generic function named foo that takes an argument T of any type:

```
fn foo<T>(arg: T) { ... }
```

- Because T has been specified as a generic type parameter using <T>, it is considered generic when used here as (arg:
  T).
- This is the case even if T has previously been defined as a struct.
- This example shows some of the syntax in action:

```
// A concrete type `A`.
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);
//            ^ Here is `Single`s first use of the type `A`.

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicitly takes `A`.
    let _s = Single(A);
    
    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`.
    // Here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t    = SingleGen(A); // Uses `A` defined at the top.
    let _i32  = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.
}

```

## Functions

- The same set of rules can be applied to functions: a type T becomes generic when preceded by <T>.
- Using generic functions sometimes requires explicitly specifying type parameters.
- This may be the case if the function is called where the return type is generic, or if the compiler doesn't have
  enough information to infer the necessary type parameters.
- A function call with explicitly specified type parameters looks like: fun::<A, B, ...>().

## Implementation

- Similar to functions, implementations require care to remain generic.

```
struct S; // Concrete type `S`
struct GenericVal<T>(T); // Generic type `GenericVal`

// impl of GenericVal where we explicitly specify type parameters:
impl GenericVal<f32> {} // Specify `f32`
impl GenericVal<S> {} // Specify `S` as defined above

// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {}

```

## Traits

- Of course traits can also be generic.
- Here we define one which reimplements the Drop trait as a generic method to drop itself and an input.

```
// Non-copyable types.
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
}
```

## Bounds

- When working with generics, the type parameters often must use traits as bounds to stipulate what functionality a type
  implements.
- For example, the following example uses the trait Display to print and so it requires T to be bound by Display; that
  is, T must implement Display.

```
// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

- Bounding restricts the generic to types that conform to the bounds. That is:

```
struct S<T: Display>(T);

// Error! `Vec<T>` does not implement `Display`. This
// specialization will fail.
let s = S(vec![1]);
```

- Another effect of bounding is that generic instances are allowed to access the methods of traits specified in the
  bounds. For example:

```
// A trait which implements the print marker: `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`. 
}
```

## Testcase: empty bounds
A consequence of how bounds work is that even if a trait doesn't include any functionality, you can still use it as a bound.
Eq and Copy are examples of such traits from the std library.
```a
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
```

## Multiple bounds
- Multiple bounds for a single type can be applied with a +. Like normal, different types are separated with ,.
```
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // TODO ^ Try uncommenting this.

    compare_types(&array, &vec);
}
```
