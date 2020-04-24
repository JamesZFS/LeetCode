/*
岛屿的最大面积
给定一个包含了一些 0 和 1 的非空二维数组 grid 。

一个 岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在水平或者竖直方向上相邻。你可以假设 grid 的四个边缘都被 0（代表水）包围着。

找到给定的二维数组中最大的岛屿面积。(如果没有岛屿，则返回面积为 0 。)

示例 1:

[
[0,0,1,0,0,0,0,1,0,0,0,0,0],
[0,0,0,0,0,0,0,1,1,1,0,0,0],
[0,1,1,0,1,0,0,0,0,0,0,0,0],
[0,1,0,0,1,1,0,0,1,0,1,0,0],
[0,1,0,0,1,1,0,0,1,1,1,0,0],
[0,0,0,0,0,0,0,0,0,0,1,0,0],
[0,0,0,0,0,0,0,1,1,1,0,0,0],
[0,0,0,0,0,0,0,1,1,0,0,0,0]
]
对于上面这个给定矩阵应返回 6。注意答案不应该是 11 ，因为岛屿只能包含水平或垂直的四个方向的 1 。

示例 2:

[[0,0,0,0,0,0,0,0]]
对于上面这个给定的矩阵, 返回 0。

注意: 给定的矩阵grid 的长度和宽度都不超过 50。
*/

use crate::array::Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        if grid.len() == 0 { return 0; }
        let (n, m) = (grid.len(), grid[0].len());
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 { // traverse
                    let mut stack = Vec::<(usize, usize)>::new();
                    let mut area = 0;
                    stack.push((i, j));
                    loop {
                        match stack.pop() {
                            None => break,
                            Some((i, j)) => {
                                if grid[i][j] == 0 { continue; } // visited
                                grid[i][j] = 0; // mark as visited
                                area += 1;
                                // For four directions:
                                if 0 < i && grid[i - 1][j] == 1 { stack.push((i - 1, j)); }
                                if 0 < j && grid[i][j - 1] == 1 { stack.push((i, j - 1)); }
                                if i < n - 1 && grid[i + 1][j] == 1 { stack.push((i + 1, j)); }
                                if j < m - 1 && grid[i][j + 1] == 1 { stack.push((i, j + 1)); }
                            }
                        }
                    }
                    ans = max(ans, area);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ], 6),
            (vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1]
            ], 4),
            (vec![vec![0, 0, 0, 0, 0, 0, 0, 0]], 0),
        ];
        for (input, output) in cases {
            assert_eq!(Solution::max_area_of_island(input), output);
        }
    }
}

