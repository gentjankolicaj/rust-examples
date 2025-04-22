## Closures

- Closures are functions that can capture the enclosing environment.
- For example, a closure that captures the x variable: | param| { param + x}

```
# Below are different closures
1. | param | {}
2. |param:type| {}
3. |param:type| -> type {}

```

- The syntax and capabilities of closures make them very convenient for on the fly usage.
- Calling a closure is exactly like calling a function.
- However, both input and return types can be inferred and input variable names must be specified.
- Other characteristics of closures include:
- using || instead of () around input variables.
- optional body delimitation ({}) for a single line expression (mandatory otherwise).
- the ability to capture the outer environment variables.
- Similar to lambdas is java.

## Capturing

- Closures are inherently flexible and will do what the functionality requires to make the closure work without
  annotation.
- This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing.
- Closures can capture variables:

```
1.by reference: &T
2.by mutable reference: &mut T
3.by value: T
```

- They preferentially capture variables by reference and only go lower when required.

## As input parameters

- While Rust chooses how to capture variables on the fly mostly without type annotation, this ambiguity is not allowed
  when writing functions.
- When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few traits,
  and they're determined by what the closure does with captured value.
- In order of decreasing restriction, they are:

```
Fn: the closure uses the captured value by reference (&T)
FnMut: the closure uses the captured value by mutable reference (&mut T)
FnOnce: the closure uses the captured value by value (T)
```

- On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.
- For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture by &T, &mut T, or
  T, but the compiler will ultimately choose based on how the captured variables are used in the closure.
- This is because if a move is possible, then any type of borrow should also be possible. Note that the reverse is not
  true.
- If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.However, &T is allowed.

## Type anonymity

- Closures succinctly capture variables from enclosing scopes.
- Does this have any consequences? It surely does.
- Observe how using a closure as a function parameter requires generics, which is necessary because of how they are
  defined:

```
// `F` must be generic.
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
```

- When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables
  inside, meanwhile implementing the functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type.
- This type is assigned to the variable which is stored until calling.
- Since this new type is of unknown type, any usage in a function will require generics.
- However, an unbounded type parameter <T> would still be ambiguous and not be allowed.
- Thus, bounding by one of the traits: Fn, FnMut, or FnOnce (which it implements) is sufficient to specify its type.

## Input functions

- Since closures may be used as arguments, you might wonder if the same can be said about functions.
- And indeed they can! If you declare a function that takes a closure as parameter, then any function that satisfies the
  trait bound of that closure can be passed as a parameter.