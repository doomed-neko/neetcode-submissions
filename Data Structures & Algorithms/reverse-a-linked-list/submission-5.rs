impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut current = head;
        let mut items: Vec<i32> = vec![];
        while let Some(node) = current {
            items.push(node.val);
            current = node.next;
        }
        items.reverse();
        let mut new_list_head: Option<Box<ListNode>> =
            Some(Box::new(ListNode::new(items.pop().unwrap())));
        for &i in items.iter().rev() {
            let mut new = ListNode::new(i);
            new.next = new_list_head;
            new_list_head = Some(Box::new(new));
        }
        new_list_head
    }
}
