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

struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut linked: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut ref_linked = &mut linked;
        let mut ref1 = &l1;
        let mut ref2 = &l2;
        let mut carry = 0;
        while !ref1.is_none() || !ref2.is_none() {
            let mut sum = 0;
            // ref1.as_ref().unwrap().val + ref2.as_ref().unwrap().val;
            if !ref1.is_none() {
                sum += ref1.as_ref().unwrap().val;
            }
            if !ref2.is_none() {
                sum += ref2.as_ref().unwrap().val;
            }
            if carry > 0 {
                sum += carry;
            }
            if sum > 9 {
                sum = sum - 10;
                carry = 1;
            } else {
                carry = 0;
            }
            ref_linked.as_mut().unwrap().val = sum; // Some(Box::new(ListNode::new(sum)));
            ref_linked = &mut ref_linked.as_mut().unwrap().next;
            if !ref1.is_none() {
                ref1 = &ref1.as_ref().unwrap().next;
            }
            if !ref2.is_none() {
                ref2 = &ref2.as_ref().unwrap().next;
            }
            if !ref1.is_none() || !ref2.is_none() || carry > 0 {
                *ref_linked = Some(Box::new(ListNode::new(0)));
            }
        }

        if carry > 0 {
            ref_linked.as_mut().unwrap().val = carry;
        }
        linked
    }
    pub fn add_2_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let refs = [&l1,&l2];
        let mut linked = Some(Box::new(ListNode::new(0)));
        let mut sum: i32 = 0;
        for mut refi in refs {    
            let mut counter = 0;
            while !refi.is_none() {
                sum += refi.as_ref().unwrap().val*10_i32.pow(counter);
                refi = &refi.as_ref().unwrap().next;
                counter += 1;
            }
        }
        println!("{}", sum);

        let mut ref_linked = &mut linked;
        while sum > 0 {
            ref_linked.as_mut().unwrap().val = sum % 10 as i32; 
            ref_linked = &mut ref_linked.as_mut().unwrap().next;
            sum /= 10;
            if sum > 0 {
                *ref_linked = Some(Box::new(ListNode::new(0)));
            }
        }
        println!("{:?}", linked);
        linked
    }
}
macro_rules! linkedlist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new($crate::ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new($crate::ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head;
            head.next
        }
    };
}
fn main() {
    // let mut linked2 = Some(Box::new(ListNode::new(9)));
    // linked2.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
    // linked2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));

    let l1 = linkedlist![0];
    let l2 = linkedlist![0];
    let mut ref_linked = &Solution::add_two_numbers(l1, l2);
    while !ref_linked.is_none() {
        print!("{}", ref_linked.as_ref().unwrap().val);
        ref_linked = &ref_linked.as_ref().unwrap().next;
        // println!("{:?}", ref_linked);
    }
}