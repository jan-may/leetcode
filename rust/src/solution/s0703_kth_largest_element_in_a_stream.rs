/**
 * [703] Kth Largest Element in a Stream
 *
 * Design a class to find the k^th largest element in a stream. Note that it is the k^th largest element in the sorted order, not the k^th distinct element.
 * Implement KthLargest class:
 * 
 * 	KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
 * 	int add(int val) Appends the integer val to the stream and returns the element representing the k^th largest element in the stream.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["KthLargest", "add", "add", "add", "add", "add"]
 * [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
 * Output
 * [null, 4, 5, 5, 8, 8]
 * Explanation
 * KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
 * kthLargest.add(3);   // return 4
 * kthLargest.add(5);   // return 5
 * kthLargest.add(10);  // return 5
 * kthLargest.add(9);   // return 8
 * kthLargest.add(4);   // return 8
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= 10^4
 * 	0 <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	-10^4 <= val <= 10^4
 * 	At most 10^4 calls will be made to add.
 * 	It is guaranteed that there will be at least k elements in the array when you search for the k^th element.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-largest-element-in-a-stream/
// discuss: https://leetcode.com/problems/kth-largest-element-in-a-stream/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct KthLargest {
        nums: Vec<i32>,
        k: usize,
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut nums = nums;
        nums.sort();
        KthLargest {
            nums,
            k: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort_unstable();
        self.nums[self.nums.len() - self.k]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_703() {
        let mut kth_largest = KthLargest::new(1, vec![-2]);
        assert_eq!(kth_largest.add(-3), -2);
        assert_eq!(kth_largest.add(0), 0);
        assert_eq!(kth_largest.add(2), 2);
        assert_eq!(kth_largest.add(-1), 2);
        assert_eq!(kth_largest.add(4), 4);

        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);

        let mut kth_largest = KthLargest::new(1, vec![]);
        assert_eq!(kth_largest.add(-3), -3);
        assert_eq!(kth_largest.add(-2), -2);
        assert_eq!(kth_largest.add(-4), -2);
        assert_eq!(kth_largest.add(0), 0);
        assert_eq!(kth_largest.add(4), 4);
    }
}
