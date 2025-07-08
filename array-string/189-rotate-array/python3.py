from typing import List

class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        def reverse(nums: List[int], start: int, end: int) -> None:
            while start < end:
                nums[start], nums[end] = nums[end], nums[start]
                start += 1
                end -= 1
        
        n = len(nums)
        k = k % n
        if k == 0:
            return

        # Reverse the entire array
        reverse(nums, 0, n - 1)
        # Reverse the first k elements
        reverse(nums, 0, k - 1)
        # Reverse remaining elements
        reverse(nums, k, n - 1)
