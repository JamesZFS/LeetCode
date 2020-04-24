/*
复原IP地址
给定一个只包含数字的字符串，复原它并返回所有可能的 IP 地址格式。

示例:

输入: "25525511135"
输出: ["255.255.11.135", "255.255.111.35"]
*/

use crate::string::Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        use std::cmp::min;
        use std::cmp::max;
        if s.len() < 4 || s.len() > 12 { return Vec::new(); }
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = Vec::new();
        // [0..i) [i..j) [j..k) [k..n) each s.t. 0 <= x <= 255
        for i in 1..=min(n - 3, 3) {
            let (p0, rem) = s.split_at(i);
            if p0.invalid() { break; }
            for j in i + 1..=min(n - 2, i + 3) {
                let (p1, rem) = rem.split_at(j - i);
                if p1.invalid() { break; }
                for k in max(j + 1, n - 3)..=min(n - 1, j + 3) { // k - j <= 3 && n - k <= 3
                    let (p2, p3) = rem.split_at(k - j);
                    if p2.invalid() { break; }
                    if p3.invalid() { continue; }
                    ans.push(format!("{}.{}.{}.{}", p0.to_string(), p1.to_string(), p2.to_string(), p3.to_string()));
                }
            }
        }
        ans
    }
}

trait Ip {
    fn to_string(&self) -> String;
    fn invalid(&self) -> bool;
}

impl Ip for &[u8] {
    #[inline]
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self).into()
    }
    #[inline]
    fn invalid(&self) -> bool {  // check if not: 0 <= self <= 255 || self == 0*
        if (self.len() >= 3 && self.to_string().as_str() > "255") || (self[0] == '0' as u8 && self.len() >= 2) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn test() {
        let cases = vec![
            ("25525511135", vec!["255.255.11.135", "255.255.111.35"]),
            ("111111111111", vec!["111.111.111.111"]),
            ("1111111111110", vec![]),
            ("111", vec![]),
            ("010010", vec!["0.10.0.10", "0.100.1.0"])
        ];
        for (input, output) in cases {
            assert_eq!(Solution::restore_ip_addresses(input.deref().into()), output);
        }
    }
}
