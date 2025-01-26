use std::collections::VecDeque;

struct RecentCounter {
    queue : VecDeque<i32>
}

impl RecentCounter {
    
    fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while *(self.queue.front().unwrap()) < t-3000 {
            self.queue.pop_front();
        }
        return self.queue.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_counter() {
        let mut counter = RecentCounter::new();
        assert_eq!(counter.ping(1), 1);    // 只有 1 個請求在範圍內
        assert_eq!(counter.ping(100), 2);  // 有 2 個請求在範圍內 (1, 100)
        assert_eq!(counter.ping(3001), 3); // 有 3 個請求在範圍內 (1, 100, 3001)
        assert_eq!(counter.ping(3002), 3); // 有 3 個請求在範圍內 (100, 3001, 3002), 1 已超出範圍
    }
}
