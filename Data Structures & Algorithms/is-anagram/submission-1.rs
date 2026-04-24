impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut c1 = s.chars().collect::<Vec<char>>();
        let mut c2 = t.chars().collect::<Vec<char>>();
        c1.sort();
        c2.sort();
        c1 == c2
    }
}
