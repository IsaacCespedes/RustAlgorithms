pub fn edit_distance(word1: String, word2: String) -> i32 {
    let len1 = word1.len();
    let len2 = word2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        dp[i][0] = i as i32;
    }

    for j in 0..=len2 {
        dp[0][j] = j as i32;
    }

    let word1_chars: Vec<char> = word1.chars().collect();
    let word2_chars: Vec<char> = word2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            if word1_chars[i - 1] == word2_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] =
                    1 + std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i - 1][j], dp[i][j - 1]));
            }
        }
    }

    dp[len1][len2]
}
