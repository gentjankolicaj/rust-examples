## Conversion

Primitive types can be converted to each other through casting.

- rust-lang addresses conversion between custom types (i.e., struct and enum) by the use of traits.
- The generic conversions will use the 'From' and 'Into' traits.
- However there are more specific ones for the more common cases, in particular when converting to and from Strings.

## From and Into

The 'From' and 'Into' traits are inherently linked, and this is actually part of its implementation.

If you are able to convert type a from type B, then it should be easy to believe that we should be able to convert type
B to type a.

- From and Into are interchangeable

- From and Into are designed to be complementary.
- We do not need to provide an implementation for both traits.
- If you have implemented the From trait for your type, Into will call it when necessary.
- Note, however, that the converse is not true: implementing Into for your type will not automatically provide it with
  an implementation of From.