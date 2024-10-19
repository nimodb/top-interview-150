impl Solution {
    pub fn romanToInt_left_to_right(s: String) -> i32 {
        let mut total = 0;
        let mut prev_value = 0;

        for char in s.chars() {
            let current_value = match char {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if current_value > prev_value {
                total += current_value - (2 * prev_value);
            } else {
                total += current_value;
            }

            prev_value = current_value;
        }

        total
    }

    pub fn romanToInt_right_to_left(s: String) -> i32 {
        let mut total = 0;
        let mut prev_value = 0;

        for char in s.chars().rev() {
            let current_value = match char {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if current_value < prev_value {
                total -= current_value;
            } else {
                total += current_value;
            }

            prev_value = current_value;
        }

        total
    }
}