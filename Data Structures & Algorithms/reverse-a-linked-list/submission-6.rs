impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut current: Option<Box<ListNode>> = head;
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = current {
            let tmp = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = tmp;
        }
        prev
    }
}
