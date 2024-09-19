class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        length = 0
        i = len(s)
        while (i>0):
            i -= 1
            if s[i] != " ":
                length += 1
            elif length != 0 :
                break
        
        return length