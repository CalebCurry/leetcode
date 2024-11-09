// 1768 - https://leetcode.com/problems/merge-strings-alternately/

pub fn main() {
    println!(
        "{}",
        Solution::merge_alternately(String::from("abcde"), String::from("pqr"))
    );
}

struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let max = if word1.len() > word2.len() {
            word1.len()
        } else {
            word2.len()
        };
        let min = if word1.len() < word2.len() {
            word1.len()
        } else {
            word2.len()
        };

        let mut string = String::from("");
        for i in 0usize..max {
            if i < word1.len() {
                string.push(word1.chars().nth(i).unwrap());
            }
            if i < word2.len() {
                string.push(word2.chars().nth(i).unwrap());
            }
        }

        return string;
    }
}
