## Modules

Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and
manage visibility (public/private) between them.

a module is a collection of items:
1.functions
2.structs
3.traits
4.impl blocks
5.other modules.

## Visibility

By default, the items in a module have private visibility, but this can be overridden with the pub modifier.
Only the public items of a module can be accessed from outside the module scope.

## Struct visibility

Structs have an extra level of visibility with their fields.
The visibility defaults to private, and can be overridden with the pub modifier.
This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of
hiding information (encapsulation).

## use declaration

The use declaration can be used to bind a full path to a new name, for easier access. It is often used like this:

```
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main() {
    my_first_function();
}
```

## super and self

The super and self keywords can be used in the path to remove ambiguity when accessing items and to prevent unnecessary
hardcoding of paths.

## File hierarchy

Modules can be mapped to a file/directory hierarchy. Let's break down the visibility example in files:

```
$ tree .
.
├── my
│   ├── inaccessible.rs
│   └── nested.rs
├── my.rs
└── split.rs

-----------------------------------------------------------------------------------
In split.rs:

// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

-----------------------------------------------------------------------------------
In my.rs:

// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
```



