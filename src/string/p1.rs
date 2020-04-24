// 无重复字符的最长子串
// 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
//
// 示例 1:
//
// 输入: "abcabcbb"
// 输出: 3
// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
// 示例 2:
//
// 输入: "bbbbb"
// 输出: 1
// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
// 示例 3:
//
// 输入: "pwwkew"
// 输出: 3
// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
// 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串

use crate::string::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        if s.len() <= 1 { return s.len() as i32; }
        let s = s.as_bytes();
        let mut ans: i32 = 0;
        for i in 0..s.len() - 1 {
            if ((s.len() - i) as i32) < ans { break; }
            let mut cur_set = [false; 256];
            let mut cur_ans = 0;
            for j in i..s.len() {
                let cur_char = s[j] as usize;
                if cur_set[cur_char] {
                    break;
                } else {
                    cur_ans += 1;
                    cur_set[cur_char] = true;
                }
            }
            ans = max(ans, cur_ans);
        };
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("".into()), 0);
        assert_eq!(Solution::length_of_longest_substring("av".into()), 2);
        assert_eq!(Solution::length_of_longest_substring("abcbdef".into()), 5);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    }
}
