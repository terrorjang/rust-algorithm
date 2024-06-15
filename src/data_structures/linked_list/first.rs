use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }

        result
    }
}

struct Node {
    elem: i32,
    next: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let mut list = List::new();

        assert_eq!(None, list.pop());

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());

        list.push(4);
        list.push(5);

        assert_eq!(Some(5), list.pop());
        assert_eq!(Some(4), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }
}
