from typing import List

class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if len(prices) < 2:
            return 0
        
        total_profit = 0
        
        for i in range(len(prices) - 1):
            if prices[i + 1] > prices[i]:
                total_profit += prices[i + 1] - prices[i]
        
        return total_profit