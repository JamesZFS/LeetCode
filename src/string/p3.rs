/*  字符串的排列
给定两个字符串 s1 和 s2，写一个函数来判断 s2 是否包含 s1 的排列。

换句话说，第一个字符串的排列之一是第二个字符串的子串。

示例1:

输入: s1 = "ab" s2 = "eidbaooo"
输出: True
解释: s2 包含 s1 的排列之一 ("ba").


示例2:

输入: s1= "ab" s2 = "eidboaoo"
输出: False


注意：

输入的字符串只包含小写字母
两个字符串的长度都在 [1, 10,000] 之间
*/

use crate::string::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() { return false; }
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let set = {
            let mut s = [0; 256];
            for &c in s1 {
                s[c as usize] += 1;
            }
            s
        };
        for i in 0..s2.len() - s1.len() + 1 {
            let mut cur_set = set.clone();
            let mut flag = true;
            for j in i..i + s1.len() {
                let c = s2[j] as usize;
                cur_set[c] -= 1;
                if cur_set[c] < 0 {
                    flag = false;
                    break;
                }
            }
            if flag {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::check_inclusion("ab".into(), "eidbaooo".into()), true);
        assert_eq!(Solution::check_inclusion("abasd".into(), "as".into()), false);
        assert_eq!(Solution::check_inclusion("a".into(), "b".into()), false);
        assert_eq!(Solution::check_inclusion("b".into(), "b".into()), true);
        assert_eq!(Solution::check_inclusion("aa".into(), "aa".into()), true);
        assert_eq!(Solution::check_inclusion("aabc".into(), "wiueoqrjncbaaxx".into()), true);
        assert_eq!(Solution::check_inclusion("aabc".into(), "wiueoqrjncabxx".into()), false);
        assert_eq!(Solution::check_inclusion("ab".into(), "eidboaoo".into()), false);
    }
}
