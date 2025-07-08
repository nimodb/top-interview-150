# 189. Rotate Array

[Rotate Array](https://leetcode.com/problems/rotate-array/description/?envType=study-plan-v2&envId=top-interview-150)

## Understating the problem

Given an integer array `nums`, we need to rotate the array to the right by `k` steps, where `k` is non-negative. The rotation must be performed in-place, modifying the input array without using extra space beyond O(1). A right rotation by `k` steps moves the last `k` elements to the front, shifting the rest of the elements to the right.

#### Example

##### Example 1

**Input:**

nums = [1,2,3,4,5,6,7], k = 3

**Output:**

[5,6,7,1,2,3,4]

**Explanation:** After rotating the array to the right by 3 steps:
- Step 1: [7,1,2,3,4,5,6]
- Step 2: [6,7,1,2,3,4,5]
- Step 3: [5,6,7,1,2,3,4]

##### Example 2

**Input:**

nums = [-1,-100,3,99], k = 2

**Output:**

[3,99,-1,-100]

**Explanation:** After rotating the array to the right by 2 steps:
- Step 1: [99,-1,-100,3]
- Step 2: [3,99,-1,-100]

## Solution Approach

To solve this problem in-place with O(1) extra space, we use the **reverse-based approach**. The idea is to reverse the entire array, then reverse the first k elements, and finally reverse the remaining elements. This effectively achieves a right rotation by k steps. Since k can be larger than the array length, we take k % n to handle the effective number of rotations.

1. **Normalize k:**
    - Compute `k = k % n` to handle cases where `k` is larger than the array length `n`.
2. **Revers the Entire Array:**
    - Reverse all elements in `nums` to get the elements in a mirrored order.
3. **Reverse First `k` elements:**
    - Reverse the first `k` elements to place the last `k` elements (from the original array) in their correct positions at the start.
4. **Reverse Remaining Elements:**
    - Reverse the elements from index `k` to the end to place the remaining elements in their correct order.
5. **Result:**
    - The array is now rotated to the tight by `k` steps, modified in-place.

### Time and Space Complexity

- **Time Complexity:** O(n), the algorithm performs three reversal:
    - Reversing the entire array: O(n)
    - Reversing three first `k` elements: O(n)
    - Reversing the remaining `n-k` elements: O(n-k) Total time is O(n + k + (n-k)) = O(n).
- **Space Complexity:** O(1), the algorithm uses only a constant amount of extra memory for variable (`start`, `end`, `k`, `n`), regardless of the input size.

## Conclusion

The reverse-base approach is an efficient and elegant solution for rotating an array in-place with O(1) extra space. By leveraging the properties of array reversal, we achieve the desired rotation without needing additional array. Both the Python and Rust implementations are concise, meet the in-place requirement, and handle all edge cases (eg., `k > n`). This approach is particularly suitable for technical interviews due ti its clarity and efficiency.