// 1768 - https://leetcode.com/problems/merge-strings-alternately/

//creates chars() once and move through with .next()

pub fn main() {
    println!(
        "{}",
        Solution::merge_alternately(String::from("aėcde"), String::from("pqr"))
    );
}

struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let max = if word1.chars().count() > word2.chars().count() {
            word1.chars().count()
        } else {
            word2.chars().count()
        };

        let mut string = String::with_capacity(word1.len() + word2.len());

        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        for i in 0usize..max {
            if i < word1.len() {
                string.push(chars1.next().unwrap());
            }
            if i < word2.len() {
                string.push(chars2.next().unwrap());
            }
        }

        return string;
    }
}
