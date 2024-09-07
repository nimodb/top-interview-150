class Solution:
    def romanToInt_left_to_right(self, s: str) -> int:
        roman_map = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}

        total = 0
        prev_value = 0
        
        for char in s:
            current_value = roman_map[char]

            if current_value > prev_value:
                total += current_value - (2 * prev_value)
            else:
                total += current_value

            prev_value = current_value

        return total
    
    def romanToInt_right_to_left(self, s: str) -> int:
        roman_map = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}

        total = 0
        prev_value = 0

        for char in reversed(s):
            current_value = roman_map[char]

            if current_value < prev_value:
                total -= current_value
            else:
                total += current_value

            prev_value = current_value

        return total
    