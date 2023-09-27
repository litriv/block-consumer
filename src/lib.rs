pub struct BlockConsumer<I, CF> {
    iter: I,
    cf: CF,
}
impl<I, CF> Iterator for BlockConsumer<I, CF>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub fn block_consumer<I, CF>(iter: I, cf: CF) -> BlockConsumer<I, CF>
where
    I: Iterator,
    CF: FnMut(I::Item) -> I::Item,
{
    BlockConsumer { iter, cf }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = [0, 0, 1, 2, 3, 0, 4, 5, 0, 0, 0, 6, 7, 8, 0, 0];
        let mut bc = block_consumer(v.into_iter(), |v| v + 1);
        assert_eq!(bc.next(), Some(6));
        assert_eq!(bc.next(), Some(9));
        assert_eq!(bc.next(), Some(21));
        assert_eq!(bc.next(), None);
    }
}
