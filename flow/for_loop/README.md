## for loops

for and range

- The 'for in' construct can be used to iterate through an Iterator.
- One of the easiest ways to create an iterator is to use the range notation a..b.
- This yields values from a (inclusive) to b (exclusive) in steps of one.

## for and iterators

- The 'for in' construct is able to interact with an Iterator in several ways.
- As discussed in the section on the Iterator trait, by default the for loop will apply the 'into_iter' function to the
  collection.
- However, this is not the only means of converting collections into iterators.

```
into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.
```

- iter - This borrows each element of the collection through each iteration.Thus leaving the collection untouched and
  available for reuse after the loop.

<br>

- into_iter - This consumes the collection so that on each iteration the exact data is provided.
- Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

<br>

- iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.