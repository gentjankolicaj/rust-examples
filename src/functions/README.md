## Functions

- Functions are declared using the fn keyword.
- Its arguments are type annotated, just like variables, and, if the function returns a value, the return type must be
  specified after an arrow ->.
- The final expression in the function will be used as return value.
- Alternatively, the return statement can be used to return a value earlier from within the function, even from inside
  loops or if statements.

## Associated functions & Methods

- Some functions are connected to a particular type.
- These come in two forms: associated functions, and methods.
- Associated functions are functions that are defined on a type generally,
- Methods are associated functions that are called on a particular instance of a type.

## Closures

- Closures are functions that can capture the enclosing environment. For example, a closure that captures the x
  variable:

```
|val| val + x
```

- The syntax and capabilities of closures make them very convenient for on the fly usage.
- Calling a closure is exactly like calling a function.
- However, both input and return types can be inferred and input variable names must be specified.

Other characteristics of closures include:

1.using || instead of () around input variables.
2.optional body delimitation ({}) for a single line expression (mandatory otherwise).
3.the ability to capture the outer environment variables.

## Higher Order Functions

- Rust provides Higher Order Functions (HOF).
- These are functions that take one or more functions and/or produce a more useful function.
- HOFs and lazy iterators give Rust its functional flavor.

## Diverging functions
- Diverging functions never return. 
- They are marked using !, which is an empty type.
```
fn foo() -> ! {
    panic!("This call never returns.");
}
```
- As opposed to all the other types, this one cannot be instantiated, because the set of all possible values this type can have is empty.
- Note that, it is different from the () type, which has exactly one possible value.
- For example, this function returns as usual, although there is no information in the return value.

```
fn some_fn() {
    ()
}

fn main() {
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");
}
```
- As opposed to this function, which will never return the control back to the caller.
```
#![feature(never_type)]

fn main() {
let x: ! = panic!("This call never returns.");
println!("You will never see this line!");
}
```

- Although this might seem like an abstract concept, it is actually very useful and often handy.
- The main advantage of this type is that it can be cast to any other type, making it versatile in situations where an exact type is required, such as in match branches.
- This flexibility allows us to write code like this:

```
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
```
