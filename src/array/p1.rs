/*
三数之和
给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。

注意：答案中不可以包含重复的三元组。

示例：

给定数组 nums = [-1, 0, 1, 2, -1, -4]，

满足要求的三元组集合为：
[
[-1, 0, 1],
[-1, -1, 2]
]
*/

use crate::array::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::{HashSet, HashMap};
        let nums = {  // num -> # occurrences
            let mut occur = HashMap::<i32, i32>::new();
            nums.iter().for_each(|&x| *occur.entry(x).or_insert(0) += 1);
            occur
        };
        let mut res = Vec::new();
        for (&a, &a_cnt) in nums.iter() {
            for (&b, &b_cnt) in nums.iter() {
                let c = -a - b;
                if a == b {
                    if a_cnt < 2 { continue; }
                    if a == c {
                        if a_cnt < 3 { continue; }
                        res.push(vec![a, b, c]);
                    } else {
                        if nums.contains_key(&c) { res.push(vec![a, b, c]); }
                    }
                } else { // a != b
                    if b == c {
                        if b_cnt < 2 { continue; }
                        res.push(vec![a, b, c]);
                    } else {
                        if nums.contains_key(&c) { res.push(vec![a, b, c]); }
                    }
                }
            }
        }
        res
    }
}

#[inline]
fn in_order(a: i32, b: i32, c: i32) -> (i32, i32, i32) {
    if a <= b {
        if b <= c {
            (a, b, c)
        } else {
            if a <= c {
                (a, c, b)
            } else {
                (c, a, b)
            }
        }
    } else {
        if a <= c {
            (b, a, c)
        } else {
            if b <= c {
                (b, c, a)
            } else {
                (c, b, a)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![-1, 0, 1, 2, -1, -4], vec![[-1, 0, 1], [-1, -1, 2]]),
            (vec![], vec![]),
        ];
        for (input, expected) in cases {
            assert_eq!(Solution::three_sum(input), expected)
        }
    }
}
