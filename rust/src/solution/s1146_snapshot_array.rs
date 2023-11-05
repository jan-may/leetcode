/**
 * [1146] Snapshot Array
 *
 * Implement a SnapshotArray that supports the following interface:
 *
 * 	SnapshotArray(int length) initializes an array-like data structure with the given length. Initially, each element equals 0.
 * 	void set(index, val) sets the element at the given index to be equal to val.
 * 	int snap() takes a snapshot of the array and returns the snap_id: the total number of times we called snap() minus 1.
 * 	int get(index, snap_id) returns the value at the given index, at the time we took the snapshot with the given snap_id
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: ["SnapshotArray","set","snap","set","get"]
 * [[3],[0,5],[],[0,6],[0,0]]
 * Output: [null,null,0,null,5]
 * Explanation:
 * SnapshotArray snapshotArr = new SnapshotArray(3); // set the length to be 3
 * snapshotArr.set(0,5);  // Set array[0] = 5
 * snapshotArr.snap();  // Take a snapshot, return snap_id = 0
 * snapshotArr.set(0,6);
 * snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5
 *  
 * Constraints:
 *
 * 	1 <= length <= 5 * 10^4
 * 	0 <= index < length
 * 	0 <= val <= 10^9
 * 	0 <= snap_id < (the total number of times we call snap())
 * 	At most 5 * 10^4 calls will be made to set, snap, and get.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/snapshot-array/
// discuss: https://leetcode.com/problems/snapshot-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct SnapshotArray {
    arr: Vec<i32>,
    map: std::collections::HashMap<i32, Vec<i32>>,
    snap_count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            arr: vec![0; length as usize],
            map: std::collections::HashMap::new(),
            snap_count: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.arr[index as usize] = val;
    }

    fn snap(&mut self) -> i32 {
        self.snap_count += 1;
        self.map.insert(self.snap_count - 1, self.arr.clone());
        self.snap_count - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.map[&snap_id][index as usize]
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1146() {}
}
