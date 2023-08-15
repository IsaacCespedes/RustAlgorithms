pub fn lcs(text1: &str, text2: &str) -> String {
    let len1 = text1.len();
    let len2 = text2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    // dp[i][j] = length of LCS of text1[0..i] and text2[0..j]
    for i in 1..=len1 {
        for j in 1..=len2 {
            if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    let mut lcs = String::new();
    let mut i = len1;
    let mut j = len2;

    // backtrack to find the LCS
    while i > 0 && j > 0 {
        if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
            lcs.push(text1.chars().nth(i - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    lcs.chars().rev().collect()
}
