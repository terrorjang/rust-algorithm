pub struct Heap {
    items: Vec<i32>,
    comparator: fn(i32, i32) -> bool,
}

impl Heap {
    pub fn new(comparator: fn(i32, i32) -> bool) -> Self {
        Self {
            items: vec![],
            comparator,
        }
    }
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let next = Some(self.items.swap_remove(0));

        if !self.is_empty() {
            // heapify down
            let mut idx = 0;
            while self.children_present(idx) {
                let ldx = self.left_child_idx(idx);
                let rdx = self.right_child_idx(idx);

                let cdx = if rdx >= self.len() {
                    ldx
                } else {
                    if (self.comparator)(self.items[ldx], self.items[rdx]) {
                        ldx
                    } else {
                        rdx
                    }
                };

                if !(self.comparator)(self.items[idx], self.items[cdx]) {
                    self.items.swap(idx, cdx);
                }
                idx = cdx;
            }
        }

        next
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    // for heapify down
    fn left_child_idx(&self, idx: usize) -> usize {
        2 * idx + 1
    }
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.len()
    }

    pub fn add(&mut self, val: i32) {
        self.items.push(val);

        // heapify up
        let mut idx = self.len() - 1;
        while let Some(pdx) = self.parent_idx(idx) {
            if (self.comparator)(self.items[idx], self.items[pdx]) {
                self.items.swap(idx, pdx);
            }

            idx = pdx;
        }
    }

    fn parent_idx(&self, idx: usize) -> Option<usize> {
        if idx > 0 {
            Some((idx - 1) / 2)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_idx() {
        let heap = Heap::new_max();
        assert_eq!(None, heap.parent_idx(0));
        assert_eq!(Some(0), heap.parent_idx(1));
        assert_eq!(Some(0), heap.parent_idx(2));
        assert_eq!(Some(1), heap.parent_idx(3));
        assert_eq!(Some(1), heap.parent_idx(4));
        assert_eq!(Some(2), heap.parent_idx(5));
        assert_eq!(Some(2), heap.parent_idx(6));
    }

    #[test]
    fn test_empy_hep() {
        let mut heap = Heap::new_max();
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(3);
        heap.add(9);
        heap.add(2);
        heap.add(1);
        heap.add(4);
        heap.add(5);
        assert_eq!(6, heap.len());
        assert_eq!(Some(9), heap.pop());
        assert_eq!(Some(5), heap.pop());
        assert_eq!(Some(4), heap.pop());
        assert_eq!(Some(3), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(1), heap.pop());
        assert_eq!(None, heap.pop());
    }
    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(3);
        heap.add(9);
        heap.add(2);
        heap.add(1);
        heap.add(4);
        heap.add(5);
        assert_eq!(6, heap.len());
        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(3), heap.pop());
        assert_eq!(Some(4), heap.pop());
        assert_eq!(Some(5), heap.pop());
        assert_eq!(Some(9), heap.pop());
        assert_eq!(None, heap.pop());
    }
}
