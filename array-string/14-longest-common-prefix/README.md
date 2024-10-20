# 14. Longest Common Prefix

[Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/?envType=study-plan-v2&envId=top-interview-150)

## Understanding the Problem

The task is to find the **longest common prefix** (LCP) among an array of strings. A prefix is sequence of characters that appear at the beginning of string. If there is no common prefix among the strings, the function should be return an empty string `""`.

### Example

#### Example 1

**Input:**

```plaintext
strs = ["flower","flow","flight"]
```

**Output:**

```plaintext
"fl"
```

#### Example 2

**Input:**

```plaintext
strs = ["dog","racecar","car"]
```

**Output:**

```plaintext
""
```

**Explanation:** There is no common prefix among the input strings.

#### Example 3

**Input:**

```plaintext
strs = ["ab","a"]
```

**Output:**

```plaintext
"a"
```

## Solution Approach

### 1. Simplest Approach

one simple way to solve the "**Longest Common Prefix**" problem is to:

1. Sort the array of strings.
2. Compare characters of the first (smallest) string with al other strings.
3. Stop the comparison when a mismatch is found and return the common prefix.

```python
def longestCommonPrefix_simplest(self, strs: List[str]) -> str:
        if not strs:
            return ""

        # Sort the strings by length
        strs.sort(key=len)
        prefix = ""

        # Compare the first string's characters with the others
        for i in range(len(strs[0])):
            for j in range(1, len(strs)):
                if strs[0][i] != strs[j][i]:
                    return prefix

            prefix += strs[0][i]

        return prefix
```

#### Time and Space Complexity

- **Time Complexity:** O(N \* M), where N is the number of strings and M is the length of the shortest string. Sorting by length takes O(N \* log(N)).

- **Space Complexity:** O(1) because no extra space is used except for the prefix string.

#### Conclusion

This simple approach sorts the strings and compares characters one by one, returning the common prefix as soon as a mismatch is found. It's a straightforward solution with reasonable performance for smaller inputs.

### 2. Iterative Approach with Build-in functions

In this Approach, we start by assuming that the "Longest Common Prefix" is the first string in the array. We then iteratively compare this prefix with each string in array and shorten it if necessary.

1. Start with the first string as the initial common prefix.
2. Compare this prefix with each subsequent string.
3. Gradually reduce the prefix length until the current string starts with the prefix.
4. If the prefix becomes empty at any point, return `""`.

```python
def longestCommonPrefix_build_in_functions(self, strs: List[str]) -> str:
        if not strs:
            return ""

        prefix = strs[0]

        for s in strs[1:]:
            while not s.startswith(prefix):
                prefix = prefix[:-1]
                if prefix == "":
                    return ""
        return prefix
```

#### Explanation

- The function first checks if the list `strs` is empty. If it is, it returns `""`.
- It starts by treating the first string as the common prefix in the list is set as the initial prefix.
- For each subsequent string, the function reduces the prefix by one character at a time until it matches the current string.
- If the prefix becomes an empty string, there is no common prefix, and the function returns `""`.

#### Time and Space Complexity

- **Time Complexity:** O(N \* M), where N is the number of strings and M is the average length of the string. In the worst case, each character in the prefix is compared with each character in every string.

- **Space Complexity:** O(1) because we use a constant amount of extra space for the prefix variable.

#### Conclusion

This solution efficiently reduces the prefix while comparing each string. It works well for moderate input size, ensuring that unnecessary comparisons are avoided once the prefix is reduced.
