/**
 * [1921] Eliminate Maximum Number of Monsters
 *
 * You are playing a video game where you are defending your city from a group of n monsters. You are given a 0-indexed integer array dist of size n, where dist[i] is the initial distance in kilometers of the i^th monster from the city.
 * The monsters walk toward the city at a constant speed. The speed of each monster is given to you in an integer array speed of size n, where speed[i] is the speed of the i^th monster in kilometers per minute.
 * You have a weapon that, once fully charged, can eliminate a single monster. However, the weapon takes one minute to charge. The weapon is fully charged at the very start.
 * You lose when any monster reaches your city. If a monster reaches the city at the exact moment the weapon is fully charged, it counts as a loss, and the game ends before you can use your weapon.
 * Return the maximum number of monsters that you can eliminate before you lose, or n if you can eliminate all the monsters before they reach the city.
 *  
 * <strong class="example">Example 1:
 *
 * Input: dist = [1,3,4], speed = [1,1,1]
 * Output: 3
 * Explanation:
 * In the beginning, the distances of the monsters are [1,3,4]. You eliminate the first monster.
 * After a minute, the distances of the monsters are [X,2,3]. You eliminate the second monster.
 * After a minute, the distances of the monsters are [X,X,2]. You eliminate the thrid monster.
 * All 3 monsters can be eliminated.
 * <strong class="example">Example 2:
 *
 * Input: dist = [1,1,2,3], speed = [1,1,1,1]
 * Output: 1
 * Explanation:
 * In the beginning, the distances of the monsters are [1,1,2,3]. You eliminate the first monster.
 * After a minute, the distances of the monsters are [X,0,1,2], so you lose.
 * You can only eliminate 1 monster.
 *
 * <strong class="example">Example 3:
 *
 * Input: dist = [3,2,4], speed = [5,3,2]
 * Output: 1
 * Explanation:
 * In the beginning, the distances of the monsters are [3,2,4]. You eliminate the first monster.
 * After a minute, the distances of the monsters are [X,0,2], so you lose.
 * You can only eliminate 1 monster.
 *
 *  
 * Constraints:
 *
 * 	n == dist.length == speed.length
 * 	1 <= n <= 10^5
 * 	1 <= dist[i], speed[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/eliminate-maximum-number-of-monsters/
// discuss: https://leetcode.com/problems/eliminate-maximum-number-of-monsters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut dist: Vec<f64> = dist.into_iter().map(|x| x as f64).collect();
        let mut speed: Vec<f64> = speed.into_iter().map(|x| x as f64).collect();
        let mut time: Vec<f64> = vec![];
        for i in 0..dist.len() {
            time.push(dist[i] / speed[i]);
        }
        time.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut result = 0;
        for i in 0..time.len() {
            if time[i] > i as f64 {
                result += 1;
            } else {
                break;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1921() {
        assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
        assert_eq!(
            Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
            1
        );
        assert_eq!(Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]), 1);
    }
}
