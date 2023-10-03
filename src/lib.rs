pub struct BlockFolder<I, DF, FF> {
    iter: I,
    delim_func: DF,
    fold_func: FF,
}
impl<I, DF, FF> BlockFolder<I, DF, FF>
where
    I: Iterator,
    DF: FnMut(&I::Item) -> bool,
    FF: FnMut(&I::Item, &I::Item) -> I::Item,
{
    pub fn new(iter: I, delim_func: DF, fold_func: FF) -> BlockFolder<I, DF, FF> {
        BlockFolder {
            iter,
            delim_func,
            fold_func,
        }
    }
    // skips until a value is found, then return that value
    fn skip(&mut self) -> Option<I::Item> {
        loop {
            let v = self.iter.next()?;
            let is_delim = (self.delim_func)(&v);
            if !is_delim {
                return Some(v);
            }
        }
    }
    fn fold(&mut self, orig: Option<I::Item>) -> Option<I::Item> {
        let mut accum = orig.unwrap();
        loop {
            match self.iter.next() {
                Some(v) => {
                    if (self.delim_func)(&v) {
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
impl<I, DF, FF> Iterator for BlockFolder<I, DF, FF>
where
    I: Iterator,
    DF: FnMut(&I::Item) -> bool,
    FF: FnMut(&I::Item, &I::Item) -> I::Item,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let orig = BlockFolder::skip(self)?;
        Some(BlockFolder::fold(self, Some(orig))?)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
