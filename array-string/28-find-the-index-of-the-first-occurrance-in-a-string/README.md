# 28. Find the Index of the First Occurrence in a String

[Find the Index of the First Occurrence in a String](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/?envType=study-plan-v2&envId=top-interview-150)

## Understating the problem

Given two strings `needle` and `haystack`, the task is to find the index of the first occurrence of `needle` in `haystack`. if `needle` is not found in `haystack`, return -1.

Both strings consist of only lowercase English characters, and the lengths of `haystack` and `needle` are between ` and 10^4. An empty needle returns 0, as per convention.


#### Example

##### Example 1

**Input:**

haystack = "sadbutsad", needle = "sad"

**Output:**

0

**Explanation:** "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

##### Example 2

**Input:**

haystack = "leetcode", needle = "leeto"

**Output:**

-1

**Explanation:** "leeto" did not occur in "leetcode", so we return -1.

## Solution Approach

The solution uses a sliding window approach to check for the presence of `needle` in `haystack`. We iterate through `haystack` and check substrings of length equal to needle. If a substring matches `needle`, we return its starting index. If no is found, we return -1.

1. If `needle` is empty, return 0 as per the problem's implicit convention.

2. Get the length of `needle` (`n`) and `haystack` (`m`). If `haystack` is empty or `n > m`, return -1 since `needle` cannot be found.

3. Iterate through `haystack` from index 0 to `m - n` (inclusive), using `range(m - n + 1)` in Python or `0..=m.saturating_sub(n)` in Rust:
    - For each index `i`, check if the substring `haystack[i:i+n]` equals `needle`.
    - If a match is found, return `i`.

4. **Result:** If no match is found after checking all possible, return -1.

*Note on Loop Range*
Initially, there was confusion about why the loop uses `range(len(haystack) - n + 1)` instead of `range(len(haystack))`. The key is that we only need to check starting indices where a substring of length `n` (length of `needle`) cat fit within `haystack`. If `m` is the length of `haystack`, the last valid starting index is `m + n`, because the substring from `i` to `i + n - 1` must not exceed `m -1`. Thus, `range(m - n + 1)` gives indices `0` to `m - n`, Using `range(m)` would check invalid indices (where `i + n > m`), leading to partial substrings that cannot match `needle`, wasting computation. For example, with `haystack = "sadbutsad"` (`m = 9`) and `needle = "sad"` (`n = 3`), checking `i = 7` given `haystack[7:10] = "ad"`, which is too short to match.

### Time and Space Complexity

- **Time Complexity:** O(m + n), where `m` is the length of `haystack` and `n` is the length of `needle`. In the worst case, we check up to `m - n + 1` substrings, and each substring comparison takes O(n).
- **Space Complexity:** O(1), as we only use a constant amount of extra space for variables, excluding the input strings.

## Conclusion

This problem tests string matching in a straightforward manner. The sliding window approach is intuitive nd easy to implement, making it suitable for interview settings. While more advance algorithms like KMP or Boyer-Moor could reduce the time complexity to O(m + n), the provided solution is efficient enough for the given constraints (up tp 10^4 characters) and is more accessible for quick implementation. The Python and Rust solutions leverage languages-specific string handling while maintaining the same logic.