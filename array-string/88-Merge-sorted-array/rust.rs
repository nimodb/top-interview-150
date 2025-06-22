impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p = m + n - 1; // Pointer for the last position in nums1
        let mut p1 = m - 1; // Pointer for the last element in nums1's original elements
        let mut p2 = n - 1; // Pointer for the last element in nums2

        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            p -= 1;
        }

        while p2 >= 0 {
            nums1[p as usize] = nums2[p2 as usize];
            p2 -= 1;
            p -= 1;
        }
    }
}