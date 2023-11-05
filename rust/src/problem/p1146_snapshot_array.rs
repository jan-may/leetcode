pub struct Solution {}

// submission codes start here

struct SnapshotArray {
    arr: Vec<Vec<(i32, i32)>>, // Use a vector of snapshots
    snap_count: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            arr: vec![vec![(0, 0)]; length as usize], // Initialize with default values
            snap_count: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let snap_id = self.snap_count;
        self.arr[index as usize].push((snap_id, val)); // Store the snapshot value
    }

    fn snap(&mut self) -> i32 {
        self.snap_count += 1;
        self.snap_count - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snapshots = &self.arr[index as usize];
        let mut left = 0;
        let mut right = snapshots.len() - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let (snap, val) = snapshots[mid];
            if snap == snap_id {
                return val; // Found the exact snapshot
            } else if snap < snap_id {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        if right >= 0 {
            snapshots[right].1 // Return the closest previous snapshot value
        } else {
            0 // No previous snapshots, return default value
        }
    }
}

// submission codes end

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_array() {
        let mut snapshot_arr = SnapshotArray::new(3);
        snapshot_arr.set(0, 5);
        assert_eq!(snapshot_arr.snap(), 0);
        snapshot_arr.set(0, 6);
        assert_eq!(snapshot_arr.get(0, 0), 5);
    }
}
