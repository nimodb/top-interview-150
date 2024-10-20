from typing import List

class Solution:
    def longestCommonPrefix_build_in_functions(self, strs: List[str]) -> str:
        if not strs:
            return ""
         
        prefix = strs[0]
        
        for s in strs[1:]:
            while not s.startswith(prefix):
                prefix = prefix[:-1]
                if prefix == "":
                    return ""
        return prefix
    

    def longestCommonPrefix_simplest(self, strs: List[str]) -> str:
        if not strs:
            return ""
        
        strs.sort(key=len)
        prefix = ""
        
        for i in range(len(strs[0])):
            for j in range(1, len(strs)):
                if strs[0][i] != strs[j][i]:
                    return prefix
            
            prefix += strs[0][i]
        
        return prefix