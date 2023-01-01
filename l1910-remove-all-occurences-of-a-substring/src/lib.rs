impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        let mut i = 0usize;

        loop {
            if let Some(slice) = s.get(i..) {
                if let Some(next_index) = slice.find(&part) {
                    let next_index = next_index + i;
                    s.replace_range(next_index..(next_index + part.len()), "");
                    i = if part.len() <= next_index {
                        next_index - part.len() + 1
                    } else {
                        0
                    };
                } else {
                    break s;
                }
            } else {
                break s;
            }
        }
    }

    #[allow(dead_code)]
    pub fn remove_occurrences_naive(s: String, part: String) -> String {
        let mut next = s.replacen(&part, "", 1);
        let mut prev = s;
        while prev != next {
            prev = next;
            next = prev.replacen(&part, "", 1);
        }
        prev
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn dab() {
        assert_eq!(
            Solution::remove_occurrences("daabcbaabcbc".to_owned(), "abc".to_owned()),
            "dab"
        );
    }

    #[test]
    fn xy() {
        assert_eq!(
            Solution::remove_occurrences("axxxxyyyyb".to_owned(), "xy".to_owned()),
            "ab"
        );
    }

    #[test]
    fn edgy() {
        assert_eq!(
            Solution::remove_occurrences("aabababa".to_owned(), "aba".to_owned()),
            "ba"
        );
    }

    #[test]
    fn what() {
        assert_eq!(
            Solution::remove_occurrences("ccctltctlltlb".to_owned(), "ctl".to_owned()),
            "b"
        );
    }
}
