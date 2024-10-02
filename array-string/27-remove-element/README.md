# 27. Remove Element

[Remove Element](https://leetcode.com/problems/remove-element/?envType=study-plan-v2&envId=top-interview-150)

## Understating the Problem

Given an integer array `nums` and an integer `val`, remove all occurrences of `val` in `nums` [in place](https://www.geeksforgeeks.org/in-place-algorithm/). The order of the elements may be changed. Then return _the number of elements in `nums` which are not equal to `val`_.

Consider the number of elements in `nums` which are not equal to `val` be `k`, to get accepted, you need to do the following things:

- Change the array `nums` such that the first `k` elements of `nums` contain the elements which are not equal to `val`. The remaining elements of `nums` are not important as well as the size of `nums`.

- Return `k`.

**Custom Judge**
The judge will test your solution with the following code:

```java
int [] nums = [...]; // Input array
int val = ...; // Value to remove
int[] expectedNums = [...]; // The expected answer with correct length.
                            // It is sorted with no values equaling val.

int k = removeElement(nums, val); // Calls your implementation

assert k == expectedNums.length;
sort(nums, 0, k); // Sort the first k elements of nums
for (int i = 0; i < actualLength; i++) {
    assert nums[i] == expectedNums[i];
}
```

If all assertions pass, then your solution will be **accepted**.

#### Examples

##### Example 1

**Input:**

```plaintext
nums [3, 2, 2, 3], val = 3
```

**Output:**

```plaintext
2, nums = [2, 2 , _, _]
```

**Explanation:** Your function should return `k = 2`, with the first two elements of `nums` being `2`. It does not matter what you leave beyond the returned `k` (hence they are underscores).

##### Example 2

**Input:**

```plaintext
nums [0, 1, 2, 2, 3, 0, 4, 2], val = 2
```

**Output:**

```plaintext
5, nums = [0, 1, 4, 0, 3, _, _, _]
```

**Explanation:** Your function should return `k = 5`, with the first five elements of `nums` containing `0, 1, 4, 0, 3`. Note that the five elements can be returned in any order. It does not matter what you leave beyond the returned k (hence they are underscores).

## Solution Approach

1. Use a two-pointer technique:
   1. One pointer `i` traverses the entire array.
   2. Another pointer `k` keeps track of the current position for non-`val` elements.
2. As we iterate through `nums`:
   1. Whenever we encounter and element that is not equal to `val`, we place it at the position `k` and increment `k`.
   2. This effectively removes all occurrences of `val` from the first `k` elements of the array.
3. Ar the end, `k` will represent the number of elements that are not equal to `val`.

### Time and Space Complexity

- **Time Complexity**: **O(n)**, where `n` is the number of elements in the `nums` array, because we iterate through the array once.
- **Space Complexing**: **O(1)**, since we are modifying the array in place and not using any additional space that scales with the input size; the modifications are made in place.

## Conclusion
This method is optimal for problems where you need to filter out elements from a list while keeping the remaining elements in their original or modified order. The approach works for various use cases, whether the array contains duplicate elements, non-contiguous occurrences of `val`, or no occurrence at all.