impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut total_profit = 0;
        for i in 0..prices.len() - 1 {
            if prices[i + 1] > prices[i] {
                total_profit += prices[i + 1] - prices[i];
            }
        }

        total_profit
    }
}