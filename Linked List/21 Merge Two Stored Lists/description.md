# 21. Merging Two Sorted Linked Lists

[Merging Two Sorted Linked Lists](https://leetcode.com/problems/merge-two-sorted-lists/description/?envType=study-plan-v2&envId=top-interview-150)
You are given the heads of two sorted linked lists `list1` and `list2`.

Merge the two lists into one **sorted** list. The list should be made by splicing together the nodes of the first two lists.

Return _the head of the merged linked list_.

## Understanding the Problem

We're given two sorted linked lists and need to merge them into a single sorted linked list. This is a classic problem that can be solved using a variety of approaches.

## Solution Approach

### Recursive Approach

1. If one list is empty, return the other list.
   ```python
     if not list1:
         return list2
     if not list2:
         return list1
   ```
2. If the head of `list1` is smaller or equal, it's appended to the merged list, and the recursion continues with the rest of `list1` and `list2`.
   ```python
    if list1.val <= list2.val:
        list1.next = self.mergeTwoListsRecursive(list1.next, list2)
        return list1
   ```
3. If the head of `head2` is smaller, it's appended to the merged list, and the recursion continues with `list1` and the rest of `list2`
   ```python
    else:
        list2.next = self.mergeTwoListsRecursive(list1, list2.next)
        return list2
   ```

### Iterative Approach

1. A dummy node is created to simplify the handling of the head of the merged list.
   ```python
   dummy = ListNode()
   ```
2. A `current` pointer is initialized to the dummy node, which will be used to build the merged list.
   ```python
   current = dummy
   ```
3. While both `list1` and `list2` are not empty:

   - Compare the values of the current nodes in `list1` and `list2`.
   - Append the node with smaller value to the merged list using the `current` pointer and update the corresponding list.
   - Advance the `current` pointer to the next node in the merged list.

   ```python
    while list1 and list2:
        if list1.val <= list2.val:
            current.next = list1
            list1 = list1.next
        else:
            current.next = list2
            list2 = list1.next

        current = current.next
   ```

4. After the loop, one of the lists might still have remaining elements. Append the remaining elements of the non-empty list to the merged list.
   ```python
   current.next = list1 or list2
   ```
5. Return the head of the merged list, which is the next node after the dummy node.
   ```python
   return dummy.next
   ```

### Time and Space Complexity

Both the recursive and iterative approaches have a time complexity of (n + m), where n and m are the lengths of the two lists. This is because each element is compared at most once.

The recursive approach has a space complexity of O(log(n + m)) due to the recursive call stack. The iterative approach has a space complexity of O(1) as it uses constant extra space.

## Conclusion

Both the recursive and iterative approaches are valid solutions for merging two sorted linked lists. The choice of approach depends on personal preference and the specific requirements of the problem.
