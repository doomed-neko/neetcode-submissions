impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }
        [
            strs.iter()
                .map(|x| x.len().to_string())
                .collect::<Vec<String>>()
                .join(","),
            "#".to_owned(),
            strs.join(""),
        ]
        .join("")
    }

    pub fn decode(s: String) -> Vec<String> {
        if s.is_empty() {
            return vec![];
        }
        let mut decoded: Vec<String> = Vec::new();
        let (lengths, encoded) = s.split_once('#').unwrap();
        let lengths = lengths
            .chars()
            .take_while(|x| x != &'#')
            .collect::<String>();
        let lengths = lengths.split(',').map(|x| x.parse::<usize>().unwrap());
        let mut i = 0;
        for j in lengths {
            decoded.push(encoded[i..i + j].to_string());
            i += j;
        }
        decoded
    }
}
