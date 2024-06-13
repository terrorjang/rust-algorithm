use std::cmp::{Ordering, PartialOrd};

pub struct BinarySearchTree<T>
where
    T: PartialOrd,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd,
{
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
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

    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    pub fn search(&self, value: T) -> bool {
        match &self.value {
            Some(key) => match key.partial_cmp(&value).unwrap() {
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
        assert_eq!(*tree.minimum().unwrap(), 0);
        assert_eq!(*tree.maximum().unwrap(), 0);
        tree.insert(10);
        assert_eq!(*tree.minimum().unwrap(), 0);
        assert_eq!(*tree.maximum().unwrap(), 10);
        tree.insert(-15);
        assert_eq!(*tree.minimum().unwrap(), -15);
        assert_eq!(*tree.maximum().unwrap(), 10);
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

    #[test]
    fn test_f64() {
        let mut tree = BinarySearchTree::new();
        tree.insert(0.);
        assert_eq!(*tree.minimum().unwrap(), 0.);
        assert_eq!(*tree.maximum().unwrap(), 0.);
        tree.insert(10.1);
        assert_eq!(*tree.minimum().unwrap(), 0.);
        assert_eq!(*tree.maximum().unwrap(), 10.1);
        tree.insert(-15.1);
        assert_eq!(*tree.minimum().unwrap(), -15.1);
        assert_eq!(*tree.maximum().unwrap(), 10.1);
    }

    #[test]
    fn search_f64() {
        let mut tree = BinarySearchTree::new();
        tree.insert(50.1);
        tree.insert(10.1);
        tree.insert(60.1);
        tree.insert(-150.1);
        tree.insert(77.1);
        tree.insert(-201.1);

        assert!(tree.search(60.1));
        assert!(!tree.search(62.1));
    }

    #[test]
    fn test_str() {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello world");
        tree.insert("test string");
        tree.insert("rust is good");
        tree.insert("c++ is not bad");
        tree.insert("java is not my type");
        tree.insert("lololol");
        tree.insert("great!");

        assert!(tree.search("rust is good"));
        assert!(!tree.search("rust is bad"));

        assert_eq!(&"c++ is not bad", tree.minimum().unwrap());
        assert_eq!(&"test string", tree.maximum().unwrap());
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERROR 3
// When use generic type

/*
error[E0369]: binary operation `<` cannot be applied to type `T`
  --> src/data_structures/binary_search_tree.rs:22:44
   |
22 |                 let target_node = if value < *key {
   |                                      ----- ^ ---- T
   |                                      |
   |                                      T
   |
help: consider restricting type parameter `T`
   |
9  | impl<T: std::cmp::PartialOrd> BinarySearchTree<T> {

SOLUTION

where
    T: Ord
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERROR 4
// When use generic type

/*
error[E0507]: cannot move out of `self.value` which is behind a shared reference
  --> src/data_structures/binary_search_tree.rs:51:21
   |
51 |             None => self.value,
   |                     ^^^^^^^^^^ move occurs because `self.value` has type `Option<T>`, which does not implement the `Copy` trait

SOLUTION
change return type
Option<T> -> Option<&T>

self.value -> self.value.as_ref()
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERROR 5
/*
error[E0277]: can't compare `&{integer}` with `{integer}`
  --> src/data_structures/binary_search_tree.rs:94:9
   |
94 |         assert_eq!(tree.maximum().unwrap(), 10);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&{integer} == {integer}`
   |
   = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider dereferencing here
  --> /Users/youngseok/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/macros/mod.rs:40:22
   |
40 |                 if !(**left_val == *right_val) {
   |                      +

SOLUTION
add dereference mark
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERROR 6
// When test f64
/*
error[E0277]: the trait bound `{float}: Ord` is not satisfied
   --> src/data_structures/binary_search_tree.rs:100:14
    |
100 |         tree.insert(0.);
    |              ^^^^^^ the trait `Ord` is not implemented for `{float}`
    |
    = help: the following other types implement trait `Ord`:
              isize
              i8
              i16
              i32
              i64
              i128
              usize
              u8
            and 4 others
note: required by a bound in `binary_search_tree::BinarySearchTree::<T>::insert`
   --> src/data_structures/binary_search_tree.rs:14:8
    |
14  |     T: Ord,
    |        ^^^ required by this bound in `BinarySearchTree::<T>::insert`
...
24  |     pub fn insert(&mut self, value: T) {
    |            ------ required by a bound in this associated function

SOLUTION
Ord to PatialOrd
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERROR 6
// When test f64

/*
error[E0599]: the method `cmp` exists for reference `&T`, but its trait bounds were not satisfied
  --> src/data_structures/binary_search_tree.rs:63:36
   |
63 |             Some(key) => match key.cmp(&value) {
   |                                    ^^^ method cannot be called on `&T` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `T: Ord`
           which is required by `&T: Ord`
           `&T: Iterator`
           which is required by `&mut &T: Iterator`
           `T: Iterator`
           which is required by `&mut T: Iterator`
   = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `cmp`, perhaps you need to restrict type parameter `T` with one of them:
   |
14 |     T: PartialOrd + Ord,
   |                   +++++
14 |     T: PartialOrd + Iterator,
   |                   ++++++++++

SOLUTION
cmp to partial_cmp
*/
