pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<'a, T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let ret = self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        });

        ret
    }

    pub fn peek(&self) -> Option<&T> {
        let ret = self.head.as_ref().map(|node| &node.elem);

        ret
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let ret = self.head.as_mut().map(|node| &mut node.elem);

        ret
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// pub enum Link {
//     Empty,
//     More(Box<Node>),
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_test() {
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

    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(None, list.peek());
        assert_eq!(None, list.peek_mut());

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(Some(&3), list.peek());
        assert_eq!(Some(&mut 3), list.peek_mut());

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(Some(&42), list.peek());
        assert_eq!(Some(42), list.pop());
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
