#[derive(Debug)]
struct MyLinkedList {
    val:i32,
    next:Option<Box<MyLinkedList>>
}
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        MyLinkedList {
            val: 1,
            next: None,
        }
    }
    
    // fn get(&self, index: i32) -> i32 {
        
    // }
    
    fn add_at_head(&mut self, val: i32) {
        let temp = self.next.take();
        let new_node = MyLinkedList {
            val: val,
            next: temp,
        };
        self.next = Some(Box::new(new_node));

    }
    
    fn add_at_tail(&mut self, val: i32) {
        let mut current = self;
        while let Some(ref mut node) = current.next {
            current = node;
        }
        current.next = Some(Box::new(MyLinkedList {
            val: val,
            next: None,
        }));
    }
    
    fn add_at_index(&self, index: i32, val: i32) {
        
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
        let mut list = MyLinkedList::new();
        assert_eq!(list.val, 1);
    }

    #[test]
    fn test_add_at_head() {
        let mut list = MyLinkedList::new();
        list.add_at_head(2);
        list.add_at_head(3);
        println!("{:?}", list);
        assert_eq!(list.next.unwrap().val, 3);
        // assert_eq!(list.next.unwrap().val, 1);
    }

    #[test]
    fn test_add_at_tail() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(2);
        list.add_at_tail(3);
        println!("{:?}", list);
        assert_eq!(list.next.unwrap().val, 2);
        // assert_eq!(list.next.unwrap().val, 1);
    }
}
