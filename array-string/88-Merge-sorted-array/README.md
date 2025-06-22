# 88. Merge Sorted Array

[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&envId=top-interview-150)

## Understating the problem

Given two integer arrays `nums1` and `nums2`, sorted in non-decreasing order, and integers `m` and `n`representing the number of elements in `nums1` and `nums1` respectively, merge `nums1` and `nums1` to form a single sorted array in non-decreasing order. The array `nums1` has a length of `m + n`, where the first `m` elements are the ones to be merged, and the last `n` elements are zeros to be ignored. The merged result is sorted in `nums1` in-place.

#### Example

##### Example 1

**Input:**

nums1 = [1,2,3,0,0,0], m = 3
nums2 = [2,5,6], n = 3

**Output:**

[1,2,2,3,5,6]

**Explanation:** Merging [1,2,3] and [2,5,6] results in [1,2,2,3,5,6], stored in `nums1`

##### Example 2

**Input:**

nums1 = [1], m = 1
nums2 = [], n = 0

**Output:**

[1]

**Explanation:** Merging [1] and [] results in [1], stored in nums1.

## Solution Approach

The solution merges the arrays in-place by staring from the end of both arrays and filling `nums1` form its last position. This avoids overwriting elements in `nums1` that need to be merged and utilizes extra space at the end of `nums1`. We use three pointers: `p` for the current position in `nums1` to place the larger element, `p1` for the last valid element in `nums1`, and `p2` for the last element in `nums2`.

1. Initialize `p = m + n -1`, `p1 = m - 1`, and `p2 = n - 1` 

2. While both `p1` and `p2` are valid (`p1 >= 0` and `p2 >= 0`):
    - If `nums1[p1] > nums2[p2]`, place `nums1[p1]` at `nums1[p]` and decrement `p1`.
    - Otherwise, place `nums2[p2]` at nums1[p] and decrement `p2`.
    - Decrement `p` after each placement.

3. If `p2 >= 0` (elements remain in `nums2`), copy them to `nums1[p]` and decrement `p2` and `p`.

4. **Result:** `nums1` contains the merged sorted array.

### Time and Space Complexity

- **Time Complexity:** O(m + n), as we traverse both arrays once, comparing and placing elements in a single pass.
- **Space Complexity:** O(1), as we merge in-place without using extra space beyond a few variables.

## Conclusion

This problem demonstrates an efficient in-place merge technique for sorted arrays, leveraging the extra space in `nums1`. By merging from the end, we avoid the need for additional space and achieve the optimal O(m + n) time complexity. The Python and Rust implementations highlight language-specific handling of array indexing and mutability while maintaining the same algorithmic logic.
