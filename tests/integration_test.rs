use block_consumer::BlockFolder;

#[test]
fn test_owned() {
    let v = [0, 0, 1, 2, 3, 0, 4, 5, 0, 0, 0, 6, 7, 8, 0, 0];
    let mut bc = BlockFolder::new(v.into_iter(), |v| *v == 0, |orig, v| v + orig);
    assert_eq!(bc.next(), Some(6));
    assert_eq!(bc.next(), Some(9));
    assert_eq!(bc.next(), Some(21));
    assert_eq!(bc.next(), None);
}

#[test]
fn test_borrowed() {
    let v = [0, 0, 1, 2, 3, 0, 4, 5, 0, 0, 0, 6, 7, 8, 0, 0];
    let mut bc = BlockFolder::new(v.iter(), |v| **v == 0, |orig, v| (**v + **orig));
    assert_eq!(bc.next(), Some(&6));
    assert_eq!(bc.next(), Some(&9));
    assert_eq!(bc.next(), Some(&21));
    assert_eq!(bc.next(), None);
}
