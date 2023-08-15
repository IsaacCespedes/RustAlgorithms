pub fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let max = i32::MAX;
    let mut dp = vec![max; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins {
            if coin <= i {
                dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - coin) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == max {
        -1
    } else {
        dp[amount as usize]
    }
}
