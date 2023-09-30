pub struct BlockConsumer<I, SF, FF> {
    iter: I,
    should_skip: SF,
    fold_func: FF,
}
impl<I, SF, FF> BlockConsumer<I, SF, FF>
where
    I: Iterator,
    SF: FnMut(&I::Item) -> bool,
    FF: FnMut(&I::Item, &I::Item) -> I::Item,
{
    pub fn new(iter: I, should_skip: SF, fold_func: FF) -> BlockConsumer<I, SF, FF> {
        BlockConsumer {
            iter,
            should_skip,
            fold_func,
        }
    }
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
                    accum = (self.fold_func)(&accum, &v)
                }
                // The item returned by skip was a singular, last item,
                // so, self.iter.next() above resulted in None.
                None => return Some(accum),
            };
        }
    }
}
impl<I, SF, FF> Iterator for BlockConsumer<I, SF, FF>
where
    I: Iterator,
    SF: FnMut(&I::Item) -> bool,
    FF: FnMut(&I::Item, &I::Item) -> I::Item,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let orig = BlockConsumer::skip(self)?;
        Some(BlockConsumer::fold(self, Some(orig))?)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
