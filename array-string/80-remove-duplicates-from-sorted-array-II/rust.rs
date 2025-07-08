impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut write = 2;
        for read in 2..nums.len() {
            if nums[read] != nums[write - 2] {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write as i32
    }
}