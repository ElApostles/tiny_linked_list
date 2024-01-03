use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, PartialEq)]
struct List<T> {
    val: T,
    next: Option<ListNode<T>>,
}

type ListNode<T> = Rc<List<T>>;

impl<T> List<T> {
    pub fn new(val: T) -> List<T> {
        List { val, next: None }
    }

    pub fn add_back(&mut self, new_node: ListNode<T>) {
        self.next = Some(new_node);
    }

    pub fn last(self) -> ListNode<T> {
        let mut last = Rc::new(self);

        while let Some(next) = last.next.clone() {
            last = next;
        }

        last
    }
}

impl<T> Deref for List<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::List;

    #[test]
    fn make_list_basic() {
        let head = List::new(37);
        assert_eq!(head.val, 37);

        let head = List::new("hello");
        assert_eq!(head.val, "hello");

        let head = List::new(List::new(4));
        assert_eq!(head.val.val, List::new(4).val);
    }

    #[test]
    fn add_back() {
        let mut head = List::new(1);
        head.add_back(Rc::new(List::new(2)));

        assert_eq!(head.next.unwrap().val, 2);
    }

    #[test]
    fn last() {
        let mut head = Rc::new(List::new(1));

        let mut last = head.next.unwrap();

        last.next = Some(Rc::new(List::new(3)));

        assert_eq!(head.last().val, 3);
    }
}
