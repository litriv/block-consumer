This Rust library implements an iterator that folds blocks of input, 
seperated by possibly multiple consecutive instances of some delimiter. 

Let's say we have the following:
```
  let v = [0,0,1,2,3,0,0,0,4,5,6,0];
```
and we want to apply a folding (aggregating) function, like `sum`,
to the blocks obtained by using `0` (or any number of consecutive `0`s 
as delimiter).  Then we can use a `BlockFolder`, which is itself
an iterator, to iterate over the sums.  In the example above, the 
first call to `next` on the `BlockFolder` will yield `Some(6)`
(because 1+2+3=6) and the second call `Some(15)` (because 4+5+6=15).
