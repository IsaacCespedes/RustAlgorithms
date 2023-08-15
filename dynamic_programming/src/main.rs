mod longest_common_subsequence;
use longest_common_subsequence::lcs;

mod coin_change;
use coin_change::coin_change;

mod edit_distance;
use edit_distance::edit_distance;

mod subset_sum;
use subset_sum::subset_sum_exists;

fn main() {
    let text1 = "abcde";
    let text2 = "ace";

    let lcs = lcs(text1, text2);
    println!("Longest Common Subsequence: {}", lcs);

    let coins = vec![1, 2, 5];
    let amount = 11;

    let min_coins = coin_change(&coins, amount);
    println!("Minimum coins needed: {}", min_coins);

    let word1 = String::from("horse");
    let word2 = String::from("ros");

    let distance = edit_distance(word1, word2);
    println!("Edit Distance: {}", distance);

    let nums = vec![3, 34, 4, 12, 5, 2];
    let target_sum = 9;

    if subset_sum_exists(&nums, target_sum) {
        println!("Subset with sum {} exists.", target_sum);
    } else {
        println!("No subset with sum {} exists.", target_sum);
    }
}
