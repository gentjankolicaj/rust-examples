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