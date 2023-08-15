pub fn subset_sum_exists(nums: &[i32], target_sum: i32) -> bool {
    let n = nums.len();
    let mut dp = vec![vec![false; (target_sum + 1) as usize]; n + 1];

    for i in 0..=n {
        dp[i][0] = true;
    }

    for i in 1..=n {
        for j in 1..=target_sum {
            if nums[i - 1] <= j {
                dp[i][j as usize] = dp[i - 1][j as usize] || dp[i - 1][(j - nums[i - 1]) as usize];
            } else {
                dp[i][j as usize] = dp[i - 1][j as usize];
            }
        }
    }

    dp[n][target_sum as usize]
}
