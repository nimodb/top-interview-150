# 169. Majority Element

[Majority Element](https://leetcode.com/problems/majority-element)

## Understating the problem

Given an array `nums` of size `n`, we need to find the majority element, which is the element that appears more that `[n / 2]` times. The problem guarantees that a majority element always exists in the array. The follow-up asks for a solution with linear time complexity (O(n)) and constant space complexity (O(1))

#### Example

##### Example 1

**Input:**

nums = [3,2,3]

**Output:**

3

**Explanation:** In the array [3,2,3], the element 3 appears twice, which is more than [3 / 2] = 1. Thus, 3 is the majority element.

##### Example 2

**Input:**

nums = [2,2,1,1,1,2,2]

**Output:**

2

**Explanation:** In the array [2,2,1,1,1,2,2], the element 2 appears four times, which is more than [7 / 3] = 3 times. Thus, 2 is the majority element.

## Solution Approach

The optimal solution uses **Boyer-Moore Voting Algorithm**, which efficiently finds the majority element in O(n) time and O(1) space. The algorithm works by maintaining a candidate for the majority element and a count. Since the majority element appears more that [n / 2] times, it will always remain as the candidate after processing the array.

1. Initialize a `candidate` and a `count` to 0.
2. Iterate through the array:
    - If `count` is 0, set the current element as the `candidate`.
    - If the current element equals the `candidate`, increment `count`.
    - Otherwise, decrement `count`.
3. **Result:** The `candidate` at the end is the majority element, as elements that are not the majority cancel each other out, leaving the majority element dominant.

### Time and Space Complexity

- **Time Complexity:** O(n), where n is the length of the array, as we traverse the array once.
- **Space Complexity:** O(1), as we only use two variables (candidate and count) regardless of input size.

## Conclusion

The Boyer-Moore Voting Algorithm provides an elegant and efficient solution to the majority element problem. By leveraging the fact that the majority element appears more than half the time, it ensures correctness while achieving linear time complexity and constant space complexity. Both the Python and Rust implementations are concise and follow the same logic, making them easy to understand and maintain. This approach is particularly valuable for technical interviews due to its optimal performance and simplicity.
