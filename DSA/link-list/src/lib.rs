use std::borrow::{Borrow, BorrowMut};

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct MyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[warn(unused_mut)]
impl<T> Node<T> {
    fn new(val: T) -> Box<Node<T>> {
        Box::new(Node {
            val: val,
            next: None,
        })
    }
}
#[warn(unused_mut)]
impl<T: std::fmt::Debug> MyLinkedList<T> {

    fn new() -> Self {
        MyLinkedList {
            head: None,
        }
    }
    
    fn add_at_head(&mut self, val: T) {
        let mut new_node = Node::new(val);

        let next = self.head.take();
        new_node.next = next;
        self.head = Some(new_node);

    }
    
    fn add_at_tail(&mut self, val: T) {
    }
    
    fn add_at_index(&self, index: i32, val: T) {
        
    }
    
    fn delete_at_index(&self, index: i32) {
        
    }
}




pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_init_linklist() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        assert_eq!(list.head.is_none(), true);
    }

    #[test]
    fn test_add_at_head() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        assert_eq!(list.head.is_none(), false);
    }

    #[test]
    fn test_add_at_tail() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_tail(1);
        assert_eq!(list.head.is_none(), false);
    }


}
