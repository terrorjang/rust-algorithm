use std::cmp::Ordering;

pub struct BinarySearchTree {
    value: Option<i32>,
    left: Option<Box<BinarySearchTree>>,
    right: Option<Box<BinarySearchTree>>,
}

impl BinarySearchTree {
    pub fn new() -> BinarySearchTree {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
        match &self.value {
            None => self.value = Some(value),
            Some(key) => {
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };

                match target_node {
                    Some(node) => {
                        node.insert(value);
                    }
                    None => {
                        let mut node = BinarySearchTree::new();
                        node.value = Some(value);
                        *target_node = Some(Box::new(node));
                    }
                }
            }
        }
    }

    pub fn minimum(&self) -> Option<i32> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value,
        }
    }
    pub fn maximum(&self) -> Option<i32> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value,
        }
    }

    pub fn search(&self, value: i32) -> bool {
        match &self.value {
            Some(key) => match key.cmp(&value) {
                Ordering::Equal => true,
                Ordering::Less => match &self.right {
                    Some(node) => node.search(value),
                    None => false,
                },
                Ordering::Greater => match &self.left {
                    Some(node) => node.search(value),
                    None => false,
                },
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        let mut tree = BinarySearchTree::new();
        tree.insert(0);
        assert_eq!(tree.minimum().unwrap(), 0);
        assert_eq!(tree.maximum().unwrap(), 0);
        tree.insert(10);
        assert_eq!(tree.minimum().unwrap(), 0);
        assert_eq!(tree.maximum().unwrap(), 10);
        tree.insert(-15);
        assert_eq!(tree.minimum().unwrap(), -15);
        assert_eq!(tree.maximum().unwrap(), 10);
    }

    #[test]
    fn search_i32() {
        let mut tree = BinarySearchTree::new();
        tree.insert(50);
        tree.insert(10);
        tree.insert(60);
        tree.insert(-150);
        tree.insert(77);
        tree.insert(-201);

        assert!(tree.search(60));
        assert!(!tree.search(62));
    }
}

// ERROR 1
/*

error[E0072]: recursive type `BinarySearchTree` has infinite size
 --> src/data_structures/binary_search_tree.rs:1:1
  |
1 | pub struct BinarySearchTree {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2 |     value: Option<i32>,
3 |     left: Option<BinarySearchTree>,
  |                  ---------------- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
3 |     left: Option<Box<BinarySearchTree>>,

SOLUTION
Wrap a recursive type field in a Box
- Option<BinarySearchTree>  ->  Option<Box<BinarySearchTree>>

WHY?
- official
 - https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes

- answer by Gemini
By default, Rust uses a stack for memory allocation. When you define a struct with a field that has the same type as the struct itself, this creates a recursive type.  This means the size of the struct cannot be determined at compile time because it depends on how deep the recursion goes. The stack however, requires a fixed size allocation upfront, which can't be done for recursive types.

Wrapping the field in a Box solves this problem. Box is a smart pointer type that allocates memory on the heap. The heap allows for dynamic memory allocation, meaning the size of the allocation can be determined at runtime. By using Box, you essentially defer the memory allocation until the program is running, when the actual size is known.


*/

// ERROR 2
/*
error[E0507]: cannot move out of `self.left` as enum variant `Some` which is behind a shared reference
  --> src/data_structures/binary_search_tree.rs:41:15
   |
41 |         match self.left {
   |               ^^^^^^^^^
42 |             Some(node) => node.minimum(),
   |                  ----
   |                  |
   |                  data moved here
   |                  move occurs because `node` has type `Box<binary_search_tree::BinarySearchTree>`, which does not implement the `Copy` trait
   |
help: consider borrowing here
   |
41 |         match &self.left {
   |               +
*/
