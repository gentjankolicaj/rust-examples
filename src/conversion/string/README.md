## To and from Strings

Converting to String

- To convert any type to a String is as simple as implementing the ToString trait for the type.
- Rather than doing so directly, you should implement the fmt::Display trait which automatically provides ToString and also allows printing the type as discussed in the section on print!.