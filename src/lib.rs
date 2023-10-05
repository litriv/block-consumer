use std::marker::PhantomData;

// pub struct BlockFolder<I, DF, FF> {
pub struct BlockFolder<I, DF, FF, FFA> {
    iter: I,
    delim_func: DF,
    fold_func: FF,
    phantom: PhantomData<FFA>,
}
// impl<I, DF, FF> BlockFolder<I, DF, FF>
impl<I, DF, FF, FFA> BlockFolder<I, DF, FF, FFA>
where
    I: Iterator,
    DF: FnMut(&I::Item) -> bool,
    FF: FnMut(FFA, &I::Item) -> FFA,
{
    pub fn new(iter: I, delim_func: DF, fold_func: FF) -> BlockFolder<I, DF, FF, FFA> {
        BlockFolder {
            iter,
            delim_func,
            fold_func,
            phantom: PhantomData,
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
    fn fold(&mut self, init: Option<FFA>) -> Option<FFA> {
        // fn fold(&mut self, init: FFA, fold_func: FF) -> FFA
        let mut accum = init.unwrap();
        loop {
            match self.iter.next() {
                Some(v) => {
                    if (self.delim_func)(&v) {
                        // We stepped inside the next separator, so we stop
                        return Some(accum);
                    }
                    accum = (self.fold_func)(accum, &v)
                }
                None => return Some(accum),
            };
        }
    }
}
// impl<I, DF, FF> Iterator for BlockFolder<I, DF, FF>
impl<I, DF, FF, FFA> Iterator for BlockFolder<I, DF, FF, FFA>
where
    I: Iterator,
    DF: FnMut(&I::Item) -> bool,
    FF: FnMut(FFA, &I::Item) -> FFA,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let init = BlockFolder::skip(self)?;
        Some(BlockFolder::fold(self, Some(init))?)
        // match BlockFolder::fold(self, Some(init)) {
        //     Some(v) => Some(v),
        //     None => None,
        // }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
