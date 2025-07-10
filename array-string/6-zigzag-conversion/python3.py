class Solution:
    def convert(self, s: str, numRows: int) -> str:
        if numRows == 1 or numRows >= len(s):
            return s
        
        rows = [[] for _ in range(numRows)]
        curr_row = 0
        step = 1 # 1 for down, -1 for up
        
        for char in s:
            rows[curr_row].append(char)
            if curr_row == 0:
                step = 1 # Move down
            elif curr_row == numRows - 1:
                step = -1 # Move up
            curr_row += step

        return ''.join(''.join(row) for row in rows)