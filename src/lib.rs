pub struct BlockFolder<I, B, D, F> {
    iter: I,
    initial: B,
    delim_func: D,
    fold_func: F,
}

impl<I, B, D, F> BlockFolder<I, B, D, F> {
    pub fn new(iter: I, initial: B, delim_func: D, fold_func: F) -> Self {
        BlockFolder {
            iter,
            initial,
            delim_func,
            fold_func,
        }
    }
}

impl<I, B, D, F> Iterator for BlockFolder<I, B, D, F>
where
    I: Iterator,
    B: Clone,
    D: FnMut(&mut I::Item) -> bool,
    F: FnMut(B, I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        let mut accum = None;

        loop {
            let Some(mut item) = self.iter.next() else { return accum; };

            if (self.delim_func)(&mut item) {
                if accum.is_none() {
                    continue;
                }

                return accum;
            }

            let lhs = accum.unwrap_or_else(|| self.initial.clone());

            accum = Some((self.fold_func)(lhs, item));
        }
    }
}

#[test]
fn test_owned() {
    let v = [0, 0, 1, 2, 3, 0, 4, 5, 0, 0, 0, 6, 7, 8, 0, 0];
    let mut bc = BlockFolder::new(
        v.into_iter(),
        0,
        |&mut v: &mut u64| v == 0,
        |acc, item| acc + item,
    );
    assert_eq!(bc.next(), Some(6));
    assert_eq!(bc.next(), Some(9));
    assert_eq!(bc.next(), Some(21));
    assert_eq!(bc.next(), None);
}

#[test]
fn test_borrowed() {
    let v = [0, 0, 1, 2, 3, 0, 4, 5, 0, 0, 0, 6, 7, 8, 0, 0];
    let mut bc = BlockFolder::new(
        v.iter(),
        0,
        |&mut &v: &mut &u64| v == 0,
        |acc, &item| acc + item,
    );
    assert_eq!(bc.next(), Some(6));
    assert_eq!(bc.next(), Some(9));
    assert_eq!(bc.next(), Some(21));
    assert_eq!(bc.next(), None);
}
