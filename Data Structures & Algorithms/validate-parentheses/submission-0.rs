impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut pars = Vec::<char>::with_capacity(s.len());
        for i in s.chars() {
            match i {
                '(' | '[' | '{' => {
                    pars.push(i);
                }
                ')' => {
                    if let Some('(') = pars.pop() {
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if let Some('[') = pars.pop() {
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if let Some('{') = pars.pop() {
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        pars.is_empty()
    }
}
