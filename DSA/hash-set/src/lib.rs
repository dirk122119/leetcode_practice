#[derive(Clone)]
struct Node {
    val: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[derive(Clone)]
struct MyHashSet {
    buckets: Vec<Link>,
    size: usize,
}

impl MyHashSet {
    fn new() -> Self {
        let size = 1009; // 1009 是質數，可以減少哈希衝突
        let buckets = vec![None; size];
        MyHashSet { buckets, size }
    }

    fn add(&mut self, key: i32) {
        let index = (key as usize) % self.size;

        // 如果桶是空的，直接插入
        if self.buckets[index].is_none() {
            self.buckets[index] = Some(Box::new(Node {
                val: key,
                next: None,
            }));
            return;
        }

        // 檢查鏈表中是否已存在該值
        let mut current = &mut self.buckets[index];
        while let Some(node) = current {
            if node.val == key {
                return; // 值已存在，不需要添加
            }
            if node.next.is_none() {
                // 在鏈表末尾添加新節點
                node.next = Some(Box::new(Node {
                    val: key,
                    next: None,
                }));
                return;
            }
            current = &mut node.next;
        }
    }

    fn remove(&mut self, key: i32) {
        let index = (key as usize) % self.size;

        let bucket = &mut self.buckets[index];
        if bucket.is_none() {
            return;
        }

        // 處理第一個節點
        if let Some(node) = bucket {
            if node.val == key {
                *bucket = node.next.take();
                return;
            }
        }

        // 處理後續節點
        let mut current = bucket.as_mut().unwrap();
        while let Some(next) = &mut current.next {
            if next.val == key {
                current.next = next.next.take();
                return;
            }
            current = current.next.as_mut().unwrap();
        }
    }

    fn contains(&self, key: i32) -> bool {
        let index = (key as usize) % self.size;
        let mut current = &self.buckets[index];
        while let Some(node) = current {
            if node.val == key {
                return true;
            }
            current = &node.next;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_hashset() {
        let set = MyHashSet::new();
        assert_eq!(set.size, 1009);
        assert_eq!(set.buckets.len(), 1009);
    }

    #[test]
    fn test_add_and_contains() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert!(set.contains(1));
        assert!(set.contains(2));
        assert!(!set.contains(3));
    }

    #[test]
    fn test_add_duplicate() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(1);  // 添加重複元素
        assert!(set.contains(1));
    }

    #[test]
    fn test_remove() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        set.remove(1);
        assert!(!set.contains(1));
        assert!(set.contains(2));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.remove(2);  // 刪除不存在的元素
        assert!(set.contains(1));
    }

    #[test]
    fn test_collision_handling() {
        let mut set = MyHashSet::new();
        // 添加會產生相同哈希值的元素
        set.add(1);
        set.add(1010);  // 1010 % 1009 = 1
        assert!(set.contains(1));
        assert!(set.contains(1010));
        
        set.remove(1);
        assert!(!set.contains(1));
        assert!(set.contains(1010));
    }

    #[test]
    fn test_operations_sequence() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        set.remove(1);
        set.add(3);
        assert!(!set.contains(1));
        assert!(set.contains(2));
        assert!(set.contains(3));
    }
}
