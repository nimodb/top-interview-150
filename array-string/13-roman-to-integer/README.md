# 13. Roman To Integer

[Roman To Integer (LeetCode)](https://leetcode.com/problems/roman-to-integer/description/?envType=study-plan-v2&envId=top-interview-150)

## Understanding the Problem

Roman numerals are represented by seven different symbols: I, V, X, L, C, D, and M.

| Symbol | Value |
| ------ | ----- |
| I      | 1     |
| V      | 5     |
| X      | 10    |
| L      | 50    |
| C      | 100   |
| D      | 500   |
| M      | 1000  |

For example, `2` is written as `II` in Roman numeral, just two onces added together. `12` is written as `XII`, which is simply `X + II`. The number `27` is written as `XXVII`, which is `XX + V + II`.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not `IIII`. Instead, the number four is written as `IV`. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as `IX`. There are six instance where subtraction is used Given a roman numeral, convert it to an integer:

- `I` can be placed before `V` (5) and `X` (10) to make 4 and 9.
- `X` can be placed before `L` (50) and `C` (100) to make 40 and 90.
- `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.

| Symbol | Value |
| ------ | ----- |
| IV     | 4     |
| IX     | 9     |
| XL     | 40    |
| XC     | 90    |
| CD     | 400   |
| CM     | 900   |

The goal of the problem is to convert Roman numeral string into an integer based on the values of these symbols and certain rules for subtraction in specific cases.

#### Examples

##### Example 1

> Input: s = "III'
> Output: 3
> Explanation: III = 3

##### Example 2

> Input: s = "LVIII'
> Output: 58
> Explanation: L = 50, V= 5, III = 3.

##### Example 3

> Input: s = "MCMXCIV'
> Output: 1994
> Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

## Solution Approach

### Left-to-Right Approach

1. Map each Roman numeral to its corresponding integer value.

```python
roman_map = {
    'I': 1,
    'V': 5,
    'X': 10,
    'L': 50,
    'C': 100,
    'D': 500,
    'M': 1000,
}
```

2. Iterate through the string of Roman numerals from left to right:

   - Add the current numeral's value to a running total.

   ```python
   current_value = roman_map[char]
   ```

   - If the current numeral's value is greater than the previous one, subtract twice the previous value (to correct for an earlier over-count).

   ```python
   if current_value > prev_value:
       total += current_value - (2 * prev_value)
   else:
       total += current_value
   ```

   - Update the previous value to the current one

   ```python
   prev_value = current_value
   ```

3. Return the total after processing the entire string.

```python
return total
```

### Right-to-Left Approach

1. Map each Roman numeral to its corresponding integer value.

```python
roman_map = {
    'I': 1,
    'V': 5,
    'X': 10,
    'L': 50,
    'C': 100,
    'D': 500,
    'M': 1000,
}
```

2. Iterate through the Roman numeral string from right to left:

   - Add the current numeral's value to a running total.

   ```python
   current_value = roman_map[char]
   ```

   - If the current numeral's value is smaller than the previous numeral (now to its right), subtract the current numeral from the total.

   ```python
   if current_value < prev_value:
        total -= current_value
    else:
        total += current_value
   ```

   - Update the previous value to the current one

   ```python
   prev_value = current_value
   ```

3. Return the total after processing the entire string.

```python
return total
```

### Time and Space Complexity

- **Time Complexity**: O(n), where n is the length of the input string `s`. We traverse the string once.
- **Space Complexing**: O(1), because the space used is constant regardless of the input size (only a fixed map is used for Roman numeral values).

## Conclusion

This problem can be solved using two different approaches:

- The left-to-right method follows a straightforward logic of adjusting for smaller numerals preceding larger ones.
- The right-to-left method is more intuitive since it handles subtractive notation more simply by comparing numerals to those to the right.

Both approaches offer efficient linear time and constant space complexity.
