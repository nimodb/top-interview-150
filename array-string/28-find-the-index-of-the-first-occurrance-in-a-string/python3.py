class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        if not needle:
            return 0
        
        n = len(needle)
        m = len(haystack)
        
        if not haystack or n > m:
            return -1
        
        for i in range(m - n + 1):
            if haystack[i:i+n] == needle:
                return i
        
        return -1