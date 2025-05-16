## Attributes

An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:

- conditional compilation of code
- set crate name, version and type (binary or library)
- disable lints (warnings)
- enable compiler features (macros, glob imports, etc.)
- link to a foreign library
- mark functions as unit tests
- mark functions that will be part of a benchmark
- attribute like macros
- Attributes look like #[outer_attribute] or #![inner_attribute], with the difference between them being where they apply.

- **#[outer_attribute]** applies to the item immediately following it. Some examples of items are: a function, a module declaration, a constant, a structure, an enum.
- Here is an example where attribute #[derive(Debug)] applies to the struct Rectangle:
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- **#![inner_attribute]** applies to the enclosing item (typically a module or a crate). In other words, this attribute is interpreted as applying to the entire scope in which it's placed.
- Here is an example where #![allow(unused_variables)] applies to the whole crate (if placed in main.rs):

```
#![allow(unused_variables)]

fn main() {
    let x = 3; // This would normally warn about an unused variable.
}

```

Attributes can take arguments with different syntaxes:

- **#[attribute = "value"]**
- **#[attribute(key = "value")]**
- **#[attribute(value)]**


Attributes can have multiple values and can be separated over multiple lines, too:

- **#[attribute(value, value2)]**
- **#[attribute(value, value2, value3,value4, value5)]**

--- 

## dead_code
- The compiler provides a dead_code lint that will warn about unused functions.
- An attribute can be used to disable the lint.

```
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}
```
- Note that in real programs, you should eliminate dead code. 
- In these examples we'll allow dead code in some places because of the interactive nature of the examples.


---

## Crates
- The crate_type attribute can be used to tell the compiler whether a crate is a binary or a library (and even which type of library), and the crate_name attribute can be used to set the name of the crate.
- However, it is important to note that both the crate_type and crate_name attributes have no effect whatsoever when using Cargo, the Rust package manager. 
- Since Cargo is used for the majority of Rust projects, this means real-world uses of crate_type and crate_name are relatively limited.

```
// This crate is a library
#![crate_type = "lib"]

// The library is named "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

- When the crate_type attribute is used, we no longer need to pass the --crate-type flag to rustc.
```
$ rustc lib.rs
$ ls lib*
library.rlib
```