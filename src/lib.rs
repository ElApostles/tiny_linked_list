use std::ops::Deref;

#[derive(Debug, PartialEq)]
struct List<T> {
    val: T,
    next: Option<Box<List<T>>>,
}

type ListNode<T> = Box<List<T>>;

impl<T> List<T> {
    pub fn new(val: T) -> Box<List<T>> {
        Box::new(List { val, next: None })
    }

    pub fn add_back(&mut self, new_node: ListNode<T>) {
        self.next = Some(new_node);
    }
}

impl<T> Deref for List<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> Iterator for List<T> {
    type Item = ListNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next
    }
}
#[cfg(test)]
mod tests {
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
        head.add_back(List::new(2));

        assert_eq!(head.next.unwrap().val, 2);
    }

    #[test]
    fn last() {
        let mut head = List::new(1);
        head.next = Some(List::new(2));
        let mut last = head.next.unwrap();
        last.next = Some(List::new(3));

        assert_eq!(head.last().unwrap().val, 3);
    }
}
