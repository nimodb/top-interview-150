# 26. Remove Duplicates from Sorted Array

[Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150)

## Understating the problem

Given an integer array `nums` sorted in **non-decreasing order**, remove the duplicates [in place](https://www.geeksforgeeks.org/in-place-algorithm/) such that each unique element appears only **once**. The relative order of the elements should be kept the **same**. Then return _the number of unique elements in_ `nums`.

Consider the number of unique elements of `nums` to be `k`, to get accepted, you need to do the following things:

- Change the array `nums` such that the first `k` elements of `nums` contain the unique elements in the order they were present in `nums` initially. The remaining elements of `nums` are not important as well as the size of `nums`.
- Return `k`.

**Custom Judge:**
The judge will test your solution with the following code:

```java
int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
```

If all assertions pass, then your solution will be **accepted**.

#### Example

##### Example 1

**Input:**

```plaintext
nums = [1,1,2]
```

**Output:**

```plaintext
2, nums = [1,2,_]
```

**Explanation:** Your function should return `k = 2`, with the first two elements of nums being `1` and `2` respectively.
It does not matter what you leave beyond the returned `k` (hence they are underscores).

##### Example 2

**Input:**

```plaintext
nums = [0,0,1,1,1,2,2,3,3,4]
```

**Output:**

```plaintext
5, nums = [0,1,2,3,4,_,_,_,_,_]
```

**Explanation:** Your function should return `k = 5`, with the first five elements of nums being `0`, `1`, `2`, `3`, and `4` respectively. It does not matter what you leave beyond the returned k (hence they are underscores).

## Solution Approach

The problem can be efficiently solved using the **two-pointer technique**. The idea behind this method is to traverse the sorted array and update it in place without needing extra space, while ensuring all unique elements are grouped at the start of the array.

1. **Pointer Initialization:**

   - We initialize two pointer: `k` and `i`.
   - The first pointer, `k`, is used to track the position of the last unique element in the array.
   - The second pointer, `i`, is used to iterate over the array.

2. **Iterating through the Array:**

   - The goal is to compare each element in the array (`nums[i]`) with the element at the position `nums[k]`, which hold the last unique element we've encountered so far.
   - During the iteration, for each element `nums[i]`, we check if it is different from the last unique elements sorted at `nums[k]`. This is because the array is sorted, so duplicates will always be next to each other.
   - If `nums[i]` is equal to `nums[k]`:
     - It means the current element is duplicate, so we simple move on the next iteration without making any changes.
   - If `nums[i]` is not equal to `nums[k]`:
     - This indicates that we've found a new unique element in this case:
       - Increment `k` by 1 to point to the next available position where the new unique element will be stored.
       - Set `nums[k]` to `nums[i]`, effectively "moving" the unique element to the correct position in the array.
       - This ensures that all unique elements are clustered at the start of the array, up to index `i`

3. **Result:**

   - Once the loop completes (after we've examined all elements in the array), `k` will represent the index of the last unique element.
   - We return `K + 1` as the final result, which gives the number of unique elements in the array.

### Time and Space Complexity

- **Time Complexity:** `O(n)`, Where `n` is the number of elements in the array. We only pass through the array once.
- **Space Complexity:** `O(1)`, since we are modifying the array in place and not using any extra place.

## Conclusion

By using the two-pointer technique, we efficiently modify the input array in place to remove duplicates while maintaining a time complexity of `O(n)` and a space complexity of `O(1)`.
