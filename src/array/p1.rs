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
        let mut res = std::collections::HashSet::<(i32, i32, i32)>::new();
        for (i, &a) in nums.iter().enumerate() {
            for (j, &b) in nums.iter().skip(i + 1).enumerate() {
                for &c in nums.iter().skip(i + j + 2) {
                    let three = in_order(a, b, c);
                    if a + b + c == 0 && !res.contains(&three) {
                        res.insert(three);
                    }
                }
            }
        }
        res.into_iter().map(|(a, b, c)| vec![a, b, c]).collect()
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
            (vec![], vec![]),
            (vec![-1, 0, 1, 2, -1, -4], vec![[-1, 0, 1], [-1, -1, 2]]),
        ];
        for (input, expected) in cases {
            assert_eq!(Solution::three_sum(input), expected)
        }
    }
}
