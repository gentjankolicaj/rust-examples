## match

- Rust provides pattern matching via the match keyword, which can be used like a C switch.
- The first matching arm is evaluated and all possible values must be covered.

## Destructuring in match

a match block can destructure items in a variety of ways.

- Destructuring Tuples
- Destructuring Arrays and Slices
- Destructuring Enums
- Destructuring Pointers
- Destructuring Structures

## pointers/ref

For pointers, a distinction needs to be made between destructuring and dereferencing as they are different concepts
which are used differently from languages like C/C++.

- Dereferencing uses *
- Destructuring uses &, ref, and ref mut