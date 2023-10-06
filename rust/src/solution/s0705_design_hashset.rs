/**
 * [705] Design HashSet
 *
 * Design a HashSet without using any built-in hash table libraries.
 * Implement MyHashSet class:
 * 
 * 	void add(key) Inserts the value key into the HashSet.
 * 	bool contains(key) Returns whether the value key exists in the HashSet or not.
 * 	void remove(key) Removes the value key in the HashSet. If key does not exist in the HashSet, do nothing.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
 * [[], [1], [2], [1], [3], [2], [2], [2], [2]]
 * Output
 * [null, null, null, true, false, null, true, null, false]
 * Explanation
 * MyHashSet myHashSet = new MyHashSet();
 * myHashSet.add(1);      // set = [1]
 * myHashSet.add(2);      // set = [1, 2]
 * myHashSet.contains(1); // return True
 * myHashSet.contains(3); // return False, (not found)
 * myHashSet.add(2);      // set = [1, 2]
 * myHashSet.contains(2); // return True
 * myHashSet.remove(2);   // set = [1]
 * myHashSet.contains(2); // return False, (already removed)
 *  
 * Constraints:
 * 
 * 	0 <= key <= 10^6
 * 	At most 10^4 calls will be made to add, remove, and contains.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-hashset/
// discuss: https://leetcode.com/problems/design-hashset/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct MyHashSet {
    capacity: usize,
    buckets: Vec<Vec<i32>>,
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        Self {
            capacity : 1000,
            buckets : vec![vec![]; 1000],
        }

    }
    
    fn add(&mut self, key: i32) {
        let hash = self.hash(key);
        if !self.buckets[hash].contains(&key) {
            self.buckets[hash].push(key);
        }
    }
    
    fn remove(&mut self, key: i32) {
        let hash = self.hash(key);
        if let Some(pos) = self.buckets[hash].iter().position(|&x| x == key) {
            self.buckets[hash].remove(pos);
        }
    }
    
    fn contains(&self, key: i32) -> bool {
        let hash = self.hash(key);
        self.buckets[hash].contains(&key)
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % self.capacity
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_705() {
        let mut obj = MyHashSet::new();
        obj.add(1);
        obj.add(2);
        assert_eq!(obj.contains(1), true);
        assert_eq!(obj.contains(3), false);
        obj.add(2);
        assert_eq!(obj.contains(2), true);
        obj.remove(2);
        assert_eq!(obj.contains(2), false);
    }
}
