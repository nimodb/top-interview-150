impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        
        fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
            while start < end {
                nums.swap(start, end);
                start += 1;
                end -= 1;
            }
        }


        let n = nums.len();
        let k = (k % n as i32) as usize;
        if k == 0 {
            return;
        }

        // Reverse entire array
        reverse(nums, 0, n - 1);
        // Reverse first k elements
        reverse(nums, 0, k - 1);
        // Reverse remaining elements
        reverse(nums, k, n - 1);
    }
}