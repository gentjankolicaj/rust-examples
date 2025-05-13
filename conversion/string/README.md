## To and from Strings

Converting to String

- To convert any type to a String is as simple as implementing the ToString trait for the type.
- Rather than doing so directly, you should implement the fmt::Display trait which automatically provides ToString and
  also allows printing the type as discussed in the section on print!.

## Parsing a String

- It's useful to convert strings into many types, but one of the more common string operations is to convert them from
  string to number.
- The idiomatic approach to this is to use the parse function and either to arrange for type inference or to specify the
  type to parse using the 'turbofish' syntax.
- This will convert the string into the type specified as long as the FromStr trait is implemented for that type.
- This is implemented for numerous types within the standard library.