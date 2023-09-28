pub struct BlockConsumer<I, SF, CF> {
    iter: I,
    should_skip: SF,
    consume: CF,
}
impl<I, SF, CF> BlockConsumer<I, SF, CF>
where
    I: Iterator,
    SF: FnMut(&I::Item) -> bool,
    CF: FnMut(&I::Item, &I::Item) -> I::Item,
{
    // skips until a value is found, then return that value
    fn skip(&mut self) -> Option<I::Item> {
        loop {
            let v = self.iter.next()?;
            if !(self.should_skip)(&v) {
                return Some(v);
            }
        }
    }
    fn fold(&mut self, orig: Option<I::Item>) -> Option<I::Item> {
        let mut accum = orig.unwrap();
        loop {
            match self.iter.next() {
                Some(v) => {
                    if (self.should_skip)(&v) {
                        // We stepped inside the next separator, so we stop
                        return Some(accum);
                    }
                    accum = (self.consume)(&accum, &v)
                }
                // The item returned by skip was a singular, last item,
                // so, self.iter.next() above resulted in None.
                None => return Some(accum),
            };
        }
    }
}
impl<I, SF, CF> Iterator for BlockConsumer<I, SF, CF>
where
    I: Iterator,
    SF: FnMut(&I::Item) -> bool,
    CF: FnMut(&I::Item, &I::Item) -> I::Item,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let orig = BlockConsumer::skip(self)?;
        Some(BlockConsumer::fold(self, Some(orig))?)
    }
}

pub fn block_consumer<I, SF, CF>(iter: I, should_skip: SF, consume: CF) -> BlockConsumer<I, SF, CF>
where
    I: Iterator,
    SF: FnMut(&I::Item) -> bool,
    CF: FnMut(&I::Item, &I::Item) -> I::Item,
{
    BlockConsumer {
        iter,
        should_skip,
        consume,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = [0, 0, 1, 2, 3, 0, 4, 5, 0, 0, 0, 6, 7, 8, 0, 0];
        let mut bc = block_consumer(v.into_iter(), |v| *v == 0, |orig, v| v + orig);
        assert_eq!(bc.next(), Some(6));
        assert_eq!(bc.next(), Some(9));
        assert_eq!(bc.next(), Some(21));
        assert_eq!(bc.next(), None);
    }
}
