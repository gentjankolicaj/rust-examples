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
