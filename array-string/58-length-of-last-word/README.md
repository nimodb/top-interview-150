# 58. Length of Last Word

[Length of Last Word](https://leetcode.com/problems/length-of-last-word/description/?envType=study-plan-v2&envId=top-interview-150)

Given a string `s` consisting of words and spaces, return _the length of the **last** word in the string._

A **word** is a maximal substring consisting of non-space characters only.

**Example 1:**

> **Input:** s = "Hello World"
>
> **Output:** 5
>
> **Explanation:** The last word is "World" with length 5.

**Example 2:**

> **Input:** s = " fly me to the moon "
>
> **Output:** 4
>
> **Explanation:** The last word is "moon" with length 4.

**Example 2:**

> **Input:** s = "luffy is still joyboy"
>
> **Output:** 6
>
> **Explanation:** The last word is "joyboy" with length 6.

## Understanding the Problem

We are given a string `s` that may contain spaces and words. Our task is return the length of the last word in the string. This string can contain with spaces before and after the last word. We need to ensure that we ignore any trailing spaces and then find the length of the last word.

## Solution Approach

1. **Initialize `length` to 0:** This will store the length of the last word.

2. **Start from the end of the string (`i = len(s)`):** We begin by traversing from the last character.

3. Traverse the string in reverse:

   - If the current character is not space (`s[i] != " "`), increment the `length`.
   - If the current character is a space and we have already started counting (`length != 0`), break the loop as we've found the last word.

4. **Return the length:** Once the loop breaks, `length` will contain the length of the last word.

### Time and space Complexity

- **Time Complexity:** O(n), where n is the length of the string. In the worst case, we may traverse the entire string.

- Space Complexity: O(1), since we are using only a few extra variables for counting and traversing the string.

## Conclusion

This approach efficiently solves the problem by traversing the string from the end, counting the characters of the last word while ignoring spaces. It has a time complexity of O(n), making it suitable for strings of large lengths.
