# 121. Best Time to Buy and Sell Stock

[Best Time to Buy and Sell Stock (LeetCode)](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/?envType=study-plan-v2&envId=top-interview-150)

You are given an array `prices` where `prices[i]` is the price of a given stock on the i<sup>th</sup> day.

You want to maximize your profit by choosing a **single day** to buy one stock and choosing a **different day in the future** to sell that stock.

_Return the maximum profit you can achieve from this transaction._ If you cannot achieve any profit, return `0`.

## Understanding the Problem

Imagine you can only buy and sell a stock once. You want to buy the stock at a low price and sell it at a higher price to maximize your profit. The problem asks you to find the maximum difference between a buying and selling price within the given price array.

## Solution Approach

### One-Pass Approach

1. Initialize two valiables: `min_price` and `max_profit`

   ```python
   mix_price = float("inf")
   max_profit = 0
   ```

2. Iterate through the `prices` array:

   ```python
   for price in prices:
   ```

   - Update `min_price` to the lowest price seen so far.

   ```python
   if price < min_price:
        min_price = price
   ```

   - Calculate the current profit by subtracting `min_price` from the current price (`prices[i]`).

   ```python
   profit = price - min_price
   ```

   - Update `max_profit` if the curent profit is greater that the current `max_profit`.

   ```python
   if profit > max_profit:
        max_profit = profit
   ```

3. Fanally, return `max_profit`. This represents the maximum profit achieveable by buying at a price lower that or equal to `min_price` and selling at a higher price.

   ```python
   return max_profit
   ```

### Time and Space Complexity

O(n), where n is the length of the prices array. This is because we iterate through the array only once.

O(1), as we use constant extra space for the min_price and max_profit variables.

## Conclusion

This one-pass approach is a simple and efficient way to find the maximum profit from buying and selling a stock once.
