## Conversion

Primitive types can be converted to each other through casting.

- Rust addresses conversion between custom types (i.e., struct and enum) by the use of traits.
- The generic conversions will use the 'From' and 'Into' traits.
- However there are more specific ones for the more common cases, in particular when converting to and from Strings.

## From and Into

The 'From' and 'Into' traits are inherently linked, and this is actually part of its implementation.

If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type
B to type A.