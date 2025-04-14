## loop

- Rust provides a loop keyword to indicate an infinite loop.

- The 'break' statement can be used to exit a loop at any time, whereas the 'continue' statement can be used to skip the
  rest of the iteration and start a new one.

## Nesting and labels

- It's possible to break or continue outer loops when dealing with nested loops.
- In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue
  statement.

## Returning from loops

- One of the uses of a loop is to retry an operation until it succeeds.
- If the operation returns a value though, you might need to pass it to the rest of the code: put it after the break,
  and it will be returned by the loop expression.