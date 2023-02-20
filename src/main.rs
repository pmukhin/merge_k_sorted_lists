#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

struct Solution;

impl Solution {
  #[inline]
  pub fn min_value(lists: &Vec<Option<Box<ListNode>>>) -> Option<(i32, usize)> {
    if lists.is_empty() {
      panic!("we can't be here");
    }

    let mut m = i32::MAX;
    let mut idx: usize = 0;
    let mut b = false;

    for i in 0..lists.len() {
      if let Some(n) = &lists[i] {
        if n.val < m {
          m = n.val;
          idx = i;
          b = true;
        }
      }
    }

    if b {
      Some((m, idx))
    } else {
      None
    }
  }

  fn recursive(lists: &mut Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if let Some((min_value, idx)) = Solution::min_value(lists) {
      let curr = lists[idx].as_ref().unwrap();
      lists[idx] = curr.next.clone();

      Some(Box::new(ListNode {
        val: min_value,
        next: Solution::recursive(lists),
      }))
    } else {
      None
    }
  }

  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
      return None;
    }
    let mut v = lists.clone();
    Solution::recursive(&mut v)
  }
}

fn main() {
  let v = vec![
    Some(Box::new(ListNode {
      val: 1,
      next: Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode::new(3))),
      })),
    })),
    Some(Box::new(ListNode {
      val: 1,
      next: Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode::new(3))),
      })),
    })),
    Some(Box::new(ListNode {
      val: 1,
      next: Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode::new(3))),
      })),
    })),
    Some(Box::new(ListNode {
      val: 1,
      next: Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode::new(3))),
      })),
    })),
    Some(Box::new(ListNode {
      val: 1,
      next: Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode::new(3))),
      })),
    })),
    Some(Box::new(ListNode {
      val: 1,
      next: Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode::new(3))),
      })),
    })),
  ];

  println!("{:?}", Solution::merge_k_lists(v));
}
