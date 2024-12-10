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
impl<T: std::fmt::Debug + Clone + Copy> MyLinkedList<T> {

    fn new() -> Self {
        MyLinkedList {
            head: None,
        }
    }

    fn get(&self, index: i32) -> Option<T> {
        let mut current = &self.head;
        let mut i = 0;
        while let Some(ref node) = current {
            if i == index {
                return Some(node.val.clone());
            }
            current = &node.next;
            i += 1;
        }
        None
    }

    
    fn add_at_head(&mut self, val: T) {
        let new_node = Box::new(Node{ val, next: self.head.take() });
        self.head = Some(new_node);
    }
    
    fn add_at_tail(&mut self, val: T) {
        let new_node = Node::new(val);
        match self.head {
            None => {
                self.head = Some(new_node);
                return;
            },
            Some(ref mut current) => {
                let mut current = current.as_mut();
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(new_node);
            }
        }
    }
    
    fn add_at_index(&mut self, index: i32, val: T) {
        if index == 0 {
            self.add_at_head(val);
            return;
        }
        let mut i = 1;
        let mut current = match self.head.as_mut(){
            Some(node) => node,
            None => return,
        };
        while i< index {
            match current.next.as_mut() {
                Some(next_node) => {
                    current = next_node;
                    i += 1;
                }
                None => return,
            }
        }
        current.next = Some(Box::new(Node{val, next: current.next.take()}));
    }
    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
        match self.head.take() {
                Some(node) => self.head = node.next,
                None => return,
            }
            return;
        }
        let mut i = 1;
        let mut current = match self.head.as_mut(){
            Some(node) => node,
            None => return,
        };
        while i< index {
            match current.next.as_mut() {
                Some(next_node) => {
                    current = next_node;
                    i += 1;
                }
                None => return,
            }
        }
        let mut next_node = match current.next.take() {
            Some(next_node) =>next_node.next,
            None => return,
        };
        current.next = next_node;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        assert_eq!(val_vec, [2,1]);
    }

    #[test]
    fn test_add_at_tail() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        assert_eq!(val_vec, [1,2]);
    }

    #[test]
    fn test_add_at_index_0() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_index(0, 1);
        list.add_at_index(0, 2);
        list.add_at_index(0, 3);

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        println!("{:?}", list);
        assert_eq!(val_vec, [3,2,1]);
    }

    #[test]
    fn test_add_at_index_1() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();

        list.add_at_index(0, 1);
        list.add_at_index(1, 2);
        list.add_at_index(1, 3);
        list.add_at_index(1, 1);

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        println!("{:?}", list);
        assert_eq!(val_vec, [1,1,3,2]);
    }
    #[test]
    fn test_delete_at_index_0_in_none_node() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.delete_at_index(0);

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        println!("{:?}", list);
        assert_eq!(val_vec, []);
    }
    
    #[test]
    fn test_delete_at_index_0() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_index(0, 1);
        list.add_at_index(0, 2);
        list.add_at_index(0, 3);
        list.delete_at_index(0);

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        println!("{:?}", list);
        assert_eq!(val_vec, [2,1]);
    }

    #[test]
    fn test_delete_at_index_1() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_index(0, 1);
        list.add_at_index(0, 2);
        list.add_at_index(0, 3);
        list.delete_at_index(1);

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        println!("{:?}", list);
        assert_eq!(val_vec, [3,1]);
    }
    #[test]
    fn test_delete_at_index_2() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_index(0, 1);
        list.add_at_index(0, 2);
        list.add_at_index(0, 3);
        list.add_at_index(0, 4);
        list.delete_at_index(2);

        let mut val_vec = vec![];
        let mut current = &list.head;
        while let Some(ref node) = current {
            val_vec.push(node.val);
            current = &node.next;
        }
        println!("{:?}", list);
        assert_eq!(val_vec, [4,3,1]);
    }

    #[test]
    fn test_get() {
        let mut list:MyLinkedList<i32> = MyLinkedList::new();
        list.add_at_index(0, 1);
        list.add_at_index(0, 2);
        list.add_at_index(0, 3);
        list.add_at_index(0, 4);
        list.get(2);

        assert_eq!(list.get(2), Some(2));
    }


}
