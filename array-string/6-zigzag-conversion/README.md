# 06. Zigzag Conversion

[Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion/description/?envType=study-plan-v2&envId=top-interview-150)

## Understating the problem

Given a string `s` and an integer `numRows`, we need to convert `s` into a zigzag patter written on `numRows` rows and then read the string line by line to produce the output. The zigzag patter moves down from row 1 to row `numRows`, then up to row 1, repeating until the string is fully processed. If `numRows = `
, the output is the original string.
#### Example

##### Example 1

**Input:**\
`s = "PAYPALISHIRING", numRows = 3`

**Output:**\
`"PAHNAPLSIIGYIR"`

**Explanation:**\
The string is written in a zigzag patters on 3 rows:
```
P   A   H   N
A P L S I I G
Y   I   R
```
Reading line by line: `PAHNAPLSIIGYIR`.

##### Example 2

**Input:**\
`s = "PAYPALISHIRING", numRows = 4`

**Output:**\
`"PINALSIGYAHRPI"`

**Explanation:**\
The string is written in a zigzag pattern on 4 rows:
```
P     I    N
A   L S  I G
Y A   H R
P     I
```
Reading line by line: `PINALSIGYAHRPI`.

##### Example 3

**Input:**\
`s = "A", numRows = 1`

**Output:**\
`"A"`

**Explanation:**\
With `numRows = 1`, the output is the same as the input string.

## Solution Approach

We use a row-by-row simulation approach to build the zigzag patter. We iterate through the input string, assigning each character to its corresponding row based on the zigzag movement (down, then up). We store characters in a list for each row and concatenate them at the end.

1. **Handle Edge Case:**
    - If `numRows = 1` or `NumRows >= len(s)`, return the original string, as no zigzag patter is formed.
2. **Simulate Zigzag:**
    - Use an array of string (or vectors in Rust) to represent each row.
    - Track the current row and direction (down or up).
    - For each character in `s`:
        - Append it to the current row.
        - If moving down and at the last row, switch to moving up.
        - If moving up and at the first row, switch to moving down.
        - Update the current row based on the direction.
3. **Combine Rows:**
    - Concatenate all rows to form the final string.

### Time and Space Complexity

- **Time Complexity:** O(n)\
  We iterate through the string of length `n` once to assign characters to rows, and then iterate through the rows to build the final string.

- **Space Complexity:** O(n)\
  We use an array of lists/vectors to store the characters, which in total will store all `n` characters of the input string.

## Conclusion

The row-by-row simulation approach effectively constructs the zigzag pattern by tracking the current row and direction. Both the Python and Rust implementations are efficient and handle all edge cases, such as `numRows = 1` or when the number of rows is at least the string length. This solution is clear and suitable for technical interviews due to its straightforward logic and ability to visualize the zigzag pattern.
