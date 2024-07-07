// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    n1.next = Self::merge_two_lists(n1.next, Some(n2));
                    Some(n1)
                } else {
                    n2.next = Self::merge_two_lists(Some(n1), n2.next);
                    Some(n2)
                }
            }
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            (None, None) => None,
        }
    }
}

fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));

    let result = Solution::merge_two_lists(list1, list2);

    println!("{:?}", result); // Output: Some(ListNode { val: 1, next: Some(ListNode { val: 1, next: Some(ListNode { val: 2, next: Some(ListNode { val: 3, next: Some(ListNode { val: 4, next: Some(ListNode { val: 4, next: None }) }) }) }) }) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: None,
                            })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(Solution::merge_two_lists(list1, list2), expected);

        let list1 = None;
        let list2 = Some(Box::new(ListNode::new(0)));
        let expected = Some(Box::new(ListNode::new(0)));
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);

        let list1 = None;
        let list2 = None;
        let expected = None;
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }
}
