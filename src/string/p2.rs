// 最长公共前缀
// 编写一个函数来查找字符串数组中的最长公共前缀。
//
// 如果不存在公共前缀，返回空字符串 ""。
//
// 示例 1:
//
// 输入: ["flower","flow","flight"]
// 输出: "fl"
// 示例 2:
//
// 输入: ["dog","racecar","car"]
// 输出: ""
// 解释: 输入不存在公共前缀。
// 说明:
//
// 所有输入只包含小写字母 a-z 。

use crate::string::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let (first, others) = match strs.split_first() {
            None => { return "".into(); }
            Some((first, others)) => (first.as_bytes(), others.iter().map(|x| x.as_bytes()).collect::<Vec<_>>()),
        };
        let mut i = 0;
        while i < first.len() && others.iter().all(|&s| i < s.len()) {
            if others.iter().all(|&s| s[i] == first[i]) { i += 1; } else { break; }
        }
        String::from_utf8_lossy(&first[0..i]).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]), "fl");
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
        assert_eq!(Solution::longest_common_prefix(vec!["abc".into()]), "abc");
        assert_eq!(Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]), "");
    }
}