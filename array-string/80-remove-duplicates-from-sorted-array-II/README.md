# 80. Remove Duplicates from Sorted Array II

[Remove Duplicates from Sorted Array II](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description/?envType=study-plan-v2&envId=top-interview-150)

## Understating the problem

Given a sorted integer array `nums` in **non-decreasing** order, we need to remove duplicates [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) such that each unique element appears **at most twice**. The **relative order** of elements should be kept the **same**.

Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the **first part** of the array `nums`. If there are `k` elements after removing the duplicates, then the first `k` elements of `nums` should hold the final result. It does not matter what you leave beyond the first `k` elements.

Return `k` *after placing the final result in the first `k` slots of `nums`.*

Do **not** allocate extra space for another array. You must do this by **modifying the input array** [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) with O(1) extra memory.

#### Example

##### Example 1

**Input:**

nums = [1,1,1,2,2,3]

**Output:**

5, nums = [1,1,2,2,3,_]

**Explanation:** Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).

##### Example 2

**Input:**

nums = [0,0,1,1,1,1,2,3,3]

**Output:**

7, nums = [0,0,1,1,2,3,3,_,_]

**Explanation:** Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).

## Solution Approach

Since the array is sorted, we can use a two-pointer approach ti track the position where the next valid element should be placed (`write`) and iterate through the array (`read`). We allow each element to appear at most twice, so we keep track of the count of occurrences for the current element.

1. **Handle Edge Case:**
    - if the array has 2 or fewer elements, return its length as no duplicates need to be removed.
2. **Initialize Variables:**
    - Use `write` to indicate the position where the next valid element should be placed.
    - Initialize `write = 2` since the first two element can alway stay (if they exist).
    Iterate with `read` starting from index 2.
3. **Iterate and Compare:**
    - For each element at `read`, compare it with element at `write - 2`.
    - If `nums[read] != nums[write - 2]`, it means we can include `nums[read]` as it either starts a new element or continues an element with fewer than two occurrences.
    - Place `nums[read]` at `write` and increment `write`.
4. **Result:**
    - Return `write`, which represents the length of the modified array (`k`).
    - The first `k` elements of `nums` contain the result.

### Time and Space Complexity

- **Time Complexity:** O(n), the algorithm iterates through the array once, performing constant-time operations for each element
- **Space Complexity:** O(1), the algorithm uses only a constant of extra memory (`write` and `read` variables), modifying the array in-place.

## Conclusion

The two-pointer approach efficiently solves the problem by leveraging the array's sorted nature. By comparing each element with the two positions before the write pointer, we ensure that each element appears at most twice while maintaining the relative order. Both Python and Rust implementations are concise. in-place, and meet the O(1) space requirement, making them ideal for technical interviews.
