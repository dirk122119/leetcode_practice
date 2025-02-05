// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev_node:Option<Box<ListNode>> = None;

        while let Some(mut current) = head {
            let next_node = current.next.take();
            current.next = prev_node;
            prev_node = Some(current);
            head = next_node;
        }
        prev_node
    }
}

fn main() {
    // 创建一个链表: 1 -> 2 -> 3 -> 4 -> 5
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None,
                    })),
                })),
            })),
        })),
    }));

    // 反转链表
    let result = Solution::reverse_list(list);

    // 打印反转后的链表
    println!("{:?}", result); // 输出: Some(ListNode { val: 5, next: Some(ListNode { val: 4, next: Some(ListNode { val: 3, next: Some(ListNode { val: 2, next: Some(ListNode { val: 1, next: None }) }) }) }) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let reversed = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::reverse_list(list), reversed);

        let list = None;
        let reversed = None;
        assert_eq!(Solution::reverse_list(list), reversed);

        let list = Some(Box::new(ListNode::new(1)));
        let reversed = Some(Box::new(ListNode::new(1)));
        assert_eq!(Solution::reverse_list(list), reversed);
    }
}
