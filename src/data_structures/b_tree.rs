struct BTree {}

impl BTree {
    fn new() -> BTree {
        BTree {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        let btree = BTree::new();
        // TODO
    }
}
