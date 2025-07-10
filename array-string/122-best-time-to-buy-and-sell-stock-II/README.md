# 122. Best Time to Buy and Sell Stock II

[Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description/?envType=study-plan-v2&envId=top-interview-150)

## Understating the problem

Given an integer array `prices` where `prices[i]` is the stock price on the `i`-th day, we need to find the maximum profit achievable by buying and selling the stock. We can hold at most one share at a time, but we can buy and sell multiple times, including buying and selling on the same day. The goal is to maximize the total profit by summing the profits from all valid transactions.

#### Example

##### Example 1

**Input:**\
`prices = [7,1,5,3,6,4]`

**Output:**\
`7`

**Explanation:** \
- Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5 - 1 = 4.
- Buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6 - 3 = 3.
- Total profit = 4 + 3 = 7.

##### Example 2

**Input:**\
`prices = [1,2,3,4,5]`

**Output:**\
`4`

**Explanation:**\
- Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5 - 1 = 4.
- Total profit = 4.

##### Example 3

**Input:**\
`prices = [7,6,4,3,1]`

**Output:**\
`0`

**Explanation:**\
- Prices are decreasing, so positive profit is possible. The maximum profit is 0.

## Solution Approach

To maximize profit, we can use a **greedy approach**. Since we can buy and sell multiple times, we collect profit from every price increase between consecutive days. If price on day `i+1` is higher than day `i`, we can buy on day `i` and sell on day `i+1` to earn the difference. This captures all possible profits, as buying and selling on consecutive days is equivalent to holding through multiple days for same total profit.

1. **Iterate Through Prices:**
    - Loop through the array from index 0 to `n-2` (where `n` is the length of `prices`).
    - For each day `i`, if `prices[i+1] > prices[i]`, add the difference `prices[i+1] - prices[i]` to the total profit.
2. **Handle Edge Case:**
    - If the array has fewer than 2 elements, return 0 since no transactions is possible.
3. **Result:**
    - The sum of all positive price difference is the maximum profit.

### Time and Space Complexity

- **Time Complexity:** O(n)\
The algorithm iterates through the array once, performing constant-time operations for each element.

- **Space Complexity:** O(1)\
The algorithm uses only a single variable (`total_profit`) to track the sum, regardless of the input size.

## Conclusion

The greedy approach is both simple and efficient for this problem, as it capitalizes on every price increase between consecutive days to maximize profit. The Python and Rust implementations are straightforward, requiring a single pass through the array with no additional data structures. This solution is ideal for technical interviews due to its clarity, efficiency, and ability to handle all edge cases, such as arrays with fewer than two elements or monotonically decreasing prices.
