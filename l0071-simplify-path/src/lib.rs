struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        for dir_or_command in path.split("/") {
            match dir_or_command {
                "" | "." => (),
                ".." => { stack.pop(); }
                dir => stack.push(dir),
            }
        }
        format!("/{}", stack.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn trailing_slash() {
        assert_eq!(Solution::simplify_path("/home/".to_owned()), "/home");
    }

    #[test]
    fn up_to_root() {
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/");
    }

    #[test]
    fn consecutive_slashes() {
        assert_eq!(Solution::simplify_path("/home//foo/".to_owned()), "/home/foo");
    }
}
